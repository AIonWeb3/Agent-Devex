use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup<'a>(env: &'a Env) -> (Address, Address, Address, AgentPayIntegrationClient<'a>) {
    env.mock_all_auths();
    let contract_id = env.register(AgentPayIntegration, ());
    let client = AgentPayIntegrationClient::new(env, &contract_id);
    let admin = Address::generate(env);
    let payer = Address::generate(env);
    let agent = Address::generate(env);
    client.initialize(&admin);
    (admin, payer, agent, client)
}

#[test]
fn deposit_records_open_escrow() {
    let env = Env::default();
    let (_admin, payer, agent, client) = setup(&env);
    let id = client.deposit(&payer, &agent, &100);
    assert_eq!(id, 0);
    let escrow = client.escrow(&id).unwrap();
    assert_eq!(escrow.amount, 100);
    assert_eq!(escrow.status, 0);
    assert_eq!(escrow.payer, payer);
    assert_eq!(escrow.agent, agent);
}

#[test]
fn execute_payment_settles_and_closes() {
    let env = Env::default();
    let (_admin, payer, agent, client) = setup(&env);
    let id = client.deposit(&payer, &agent, &50);
    client.execute_payment(&id, &agent);
    let escrow = client.escrow(&id).unwrap();
    assert_eq!(escrow.status, 1);
    assert_eq!(client.paid(&agent), 50);
}

#[test]
fn refund_closes_without_settlement() {
    let env = Env::default();
    let (_admin, payer, agent, client) = setup(&env);
    let id = client.deposit(&payer, &agent, &25);
    client.refund(&id, &payer);
    let escrow = client.escrow(&id).unwrap();
    assert_eq!(escrow.status, 2);
    assert_eq!(client.paid(&agent), 0);
}
