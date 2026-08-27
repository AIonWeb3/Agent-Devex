#![no_std]
//! AgentPay integration contract template.
//!
//! `AgentPay` and `AgentGuard` here are local interfaces that stand in for a future
//! shared crate. Swap the modules for `use agent_pay::...` / `use agent_guard::...`
//! when those packages are published.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

/// Storage keys for dummy settlement + last action.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Allowlist(Address),
    Paid(Address),
    LastAction,
}

/// AgentGuard: verify the calling agent is allowed to act on-chain.
pub struct AgentGuard;

impl AgentGuard {
    pub fn allow(env: &Env, agent: &Address) {
        env.storage()
            .instance()
            .set(&DataKey::Allowlist(agent.clone()), &true);
    }

    /// Dummy identity check: agent must be on the allowlist (set via `allow_agent`).
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
    /// Dummy settlement: require amount > 0 and accumulate paid units per agent.
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
}
