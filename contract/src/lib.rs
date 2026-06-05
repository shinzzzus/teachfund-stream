#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, token, Address, Env, String};

use crate::errors::ContractError;
use crate::events::{AttestedEvent, CreatedEvent, FundedEvent, ReleasedEvent};
use crate::types::PaymentRecord;

#[contract]
pub struct TeachFundStream;

#[contractimpl]
impl TeachFundStream {
    pub fn __constructor(env: Env, admin: Address, asset: Address, project_name: String) {
        storage::set_admin(&env, &admin);
        storage::set_asset(&env, &asset);
        storage::set_project_name(&env, &project_name);
        storage::set_total_locked(&env, 0i128);
    }

    pub fn open_course(env: Env, id: String, owner: Address, target: i128) -> Result<PaymentRecord, ContractError> {
        if target <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        owner.require_auth();
        if storage::get_record(&env, &id).is_some() {
            return Err(ContractError::AlreadyExists);
        }

        let record = PaymentRecord {
            id: id.clone(),
            owner: owner.clone(),
            target,
            funded: 0i128,
            score: 0u32,
            released: 0i128,
            status: String::from_str(&env, "opened"),
            updated_ledger: env.ledger().sequence(),
        };
        storage::set_record(&env, &id, &record);
        CreatedEvent { id, owner, target }.publish(&env);
        Ok(record)
    }

    pub fn fund_course(env: Env, id: String, from: Address, amount: i128) -> Result<i128, ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        from.require_auth();

        let mut record = storage::get_record(&env, &id).ok_or(ContractError::NotFound)?;
        let asset = storage::get_asset(&env);
        let token_client = token::Client::new(&env, &asset);
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        record.funded += amount;
        record.status = String::from_str(&env, "funded");
        record.updated_ledger = env.ledger().sequence();
        storage::set_record(&env, &id, &record);
        storage::set_contribution(&env, &from, storage::get_contribution(&env, &from) + amount);
        storage::set_total_locked(&env, storage::get_total_locked(&env) + amount);
        FundedEvent { id, from, amount }.publish(&env);
        Ok(record.funded)
    }

    pub fn verify_completion(env: Env, id: String, score: u32, status: String) -> Result<PaymentRecord, ContractError> {
        Self::require_admin(&env);
        if score > 100 {
            return Err(ContractError::InvalidScore);
        }
        let mut record = storage::get_record(&env, &id).ok_or(ContractError::NotFound)?;
        record.score = score;
        record.status = status;
        record.updated_ledger = env.ledger().sequence();
        storage::set_record(&env, &id, &record);
        AttestedEvent { id, score }.publish(&env);
        Ok(record)
    }

    pub fn release_grant(env: Env, id: String, to: Address, amount: i128) -> Result<i128, ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        Self::require_admin(&env);

        let mut record = storage::get_record(&env, &id).ok_or(ContractError::NotFound)?;
        let available = record.funded - record.released;
        if amount > available {
            return Err(ContractError::InsufficientEscrow);
        }

        let asset = storage::get_asset(&env);
        let token_client = token::Client::new(&env, &asset);
        token_client.transfer(&env.current_contract_address(), &to, &amount);

        record.released += amount;
        record.status = String::from_str(&env, "released");
        record.updated_ledger = env.ledger().sequence();
        storage::set_record(&env, &id, &record);
        storage::set_total_locked(&env, storage::get_total_locked(&env) - amount);
        ReleasedEvent { id, to, amount }.publish(&env);
        Ok(record.released)
    }

    pub fn get_record(env: Env, id: String) -> Result<PaymentRecord, ContractError> {
        storage::get_record(&env, &id).ok_or(ContractError::NotFound)
    }

    pub fn contribution(env: Env, owner: Address) -> i128 {
        storage::get_contribution(&env, &owner)
    }

    pub fn total_locked(env: Env) -> i128 {
        storage::get_total_locked(&env)
    }

    pub fn project_name(env: Env) -> String {
        storage::get_project_name(&env)
    }

    fn require_admin(env: &Env) {
        let admin = storage::get_admin(env);
        admin.require_auth();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn constructor_sets_project_name() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let asset = Address::generate(&env);
        let name = String::from_str(&env, "TeachFund Stream");
        let contract_id = env.register(TeachFundStream, (admin, asset, name.clone()));
        let client = TeachFundStreamClient::new(&env, &contract_id);

        assert_eq!(client.project_name(), name);
        assert_eq!(client.total_locked(), 0);
    }

    #[test]
    fn creates_domain_record() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let asset = Address::generate(&env);
        let name = String::from_str(&env, "TeachFund Stream");
        let contract_id = env.register(TeachFundStream, (admin, asset, name));
        let client = TeachFundStreamClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let id = String::from_str(&env, "record-001");
        let record = client.open_course(&id, &owner, &1000);

        assert_eq!(record.id, id);
        assert_eq!(record.owner, owner);
        assert_eq!(record.target, 1000);
        assert_eq!(record.funded, 0);
    }
}
