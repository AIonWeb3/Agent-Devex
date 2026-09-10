#![no_std]
//! AgentPay integration + task-escrow contract template.
//!
//! `AgentPay` and `AgentGuard` are local interfaces. Escrow helpers (`deposit`,
//! `execute_payment`, `refund`) record task funding without a live token transfer.

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

/// Storage keys for settlement, allowlist, and escrow.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Allowlist(Address),
    Paid(Address),
    LastAction,
    NextEscrow,
    Escrow(u64),
}

/// Open escrow until `execute_payment` or `refund`.
#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub payer: Address,
    pub agent: Address,
    pub amount: i128,
    /// 0 = open, 1 = executed, 2 = refunded
    pub status: u32,
}

/// AgentGuard: verify the calling agent is allowed to act on-chain.
pub struct AgentGuard;

impl AgentGuard {
    pub fn allow(env: &Env, agent: &Address) {
        env.storage()
            .instance()
            .set(&DataKey::Allowlist(agent.clone()), &true);
    }

    pub fn assert_authorized(env: &Env, agent: &Address) {
        agent.require_auth();
        let ok: bool = env
            .storage()
            .instance()
            .get(&DataKey::Allowlist(agent.clone()))
            .unwrap_or(false);
        if !ok {
            panic!("agent not authorized");
        }
    }
}

/// AgentPay: settle a payment before a state-changing agent action.
pub struct AgentPay;

impl AgentPay {
    pub fn settle(env: &Env, payer: &Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }
        let key = DataKey::Paid(payer.clone());
        let prev: i128 = env.storage().instance().get(&key).unwrap_or(0);
        env.storage().instance().set(&key, &(prev + amount));
    }
}

#[contract]
pub struct AgentPayIntegration;

#[contractimpl]
impl AgentPayIntegration {
    /// Bind an admin that may allowlist agents. Requires the admin signature.
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NextEscrow, &0u64);
    }

    pub fn allow_agent(env: Env, admin: Address, agent: Address) {
        admin.require_auth();
        AgentGuard::allow(&env, &agent);
    }

    /// Verify identity, settle payment, then record the action.
    pub fn execute_agent_action(env: Env, agent: Address, action_id: Symbol, amount: i128) {
        AgentGuard::assert_authorized(&env, &agent);
        AgentPay::settle(&env, &agent, amount);
        env.storage()
            .instance()
            .set(&DataKey::LastAction, &action_id);
    }

    pub fn last_action(env: Env) -> Option<Symbol> {
        env.storage().instance().get(&DataKey::LastAction)
    }

    pub fn paid(env: Env, agent: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Paid(agent))
            .unwrap_or(0)
    }

    /// Fund an AI task escrow. Payer must sign.
    pub fn deposit(env: Env, payer: Address, agent: Address, amount: i128) -> u64 {
        payer.require_auth();
        if amount <= 0 {
            panic!("amount must be positive");
        }
        let id: u64 = env.storage().instance().get(&DataKey::NextEscrow).unwrap_or(0);
        let escrow = Escrow {
            payer: payer.clone(),
            agent: agent.clone(),
            amount,
            status: 0,
        };
        env.storage().instance().set(&DataKey::Escrow(id), &escrow);
        env.storage().instance().set(&DataKey::NextEscrow, &(id + 1));
        env.events()
            .publish((symbol_short!("deposited"), id), (payer, agent, amount));
        id
    }

    /// Release escrow to the agent after task completion. Agent must sign.
    pub fn execute_payment(env: Env, escrow_id: u64, agent: Address) {
        agent.require_auth();
        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("unknown escrow"));
        if escrow.status != 0 {
            panic!("escrow not open");
        }
        if escrow.agent != agent {
            panic!("agent mismatch");
        }
        escrow.status = 1;
        env.storage()
            .instance()
            .set(&DataKey::Escrow(escrow_id), &escrow);
        AgentPay::settle(&env, &agent, escrow.amount);
        env.events().publish(
            (symbol_short!("executed"), escrow_id),
            (escrow.payer, agent, escrow.amount),
        );
    }

    /// Return escrow to the payer if the task failed or was cancelled. Payer must sign.
    pub fn refund(env: Env, escrow_id: u64, payer: Address) {
        payer.require_auth();
        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("unknown escrow"));
        if escrow.status != 0 {
            panic!("escrow not open");
        }
        if escrow.payer != payer {
            panic!("payer mismatch");
        }
        escrow.status = 2;
        env.storage()
            .instance()
            .set(&DataKey::Escrow(escrow_id), &escrow);
        env.events().publish(
            (symbol_short!("refunded"), escrow_id),
            (payer, escrow.agent, escrow.amount),
        );
    }

    pub fn escrow(env: Env, escrow_id: u64) -> Option<Escrow> {
        env.storage().instance().get(&DataKey::Escrow(escrow_id))
    }
}

#[cfg(test)]
mod test;
