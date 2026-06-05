use soroban_sdk::{contracttype, Address, Env, String};

use crate::types::PaymentRecord;

const INSTANCE_MIN_TTL: u32 = 17280;
const INSTANCE_EXTEND_TO: u32 = 518400;
const PERSISTENT_MIN_TTL: u32 = 17280;
const PERSISTENT_EXTEND_TO: u32 = 518400;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Asset,
    ProjectName,
    TotalLocked,
    Record(String),
    Contribution(Address),
}

fn extend_instance(env: &Env) {
    env.storage().instance().extend_ttl(INSTANCE_MIN_TTL, INSTANCE_EXTEND_TO);
}

fn extend_persistent(env: &Env, key: &DataKey) {
    env.storage().persistent().extend_ttl(key, PERSISTENT_MIN_TTL, PERSISTENT_EXTEND_TO);
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
    extend_instance(env);
}

pub fn get_admin(env: &Env) -> Address {
    extend_instance(env);
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_asset(env: &Env, asset: &Address) {
    env.storage().instance().set(&DataKey::Asset, asset);
    extend_instance(env);
}

pub fn get_asset(env: &Env) -> Address {
    extend_instance(env);
    env.storage().instance().get(&DataKey::Asset).unwrap()
}

pub fn set_project_name(env: &Env, name: &String) {
    env.storage().instance().set(&DataKey::ProjectName, name);
    extend_instance(env);
}

pub fn get_project_name(env: &Env) -> String {
    extend_instance(env);
    env.storage().instance().get(&DataKey::ProjectName).unwrap()
}

pub fn set_total_locked(env: &Env, amount: i128) {
    env.storage().instance().set(&DataKey::TotalLocked, &amount);
    extend_instance(env);
}

pub fn get_total_locked(env: &Env) -> i128 {
    extend_instance(env);
    env.storage().instance().get(&DataKey::TotalLocked).unwrap_or(0)
}

pub fn set_record(env: &Env, id: &String, record: &PaymentRecord) {
    let key = DataKey::Record(id.clone());
    env.storage().persistent().set(&key, record);
    extend_persistent(env, &key);
}

pub fn get_record(env: &Env, id: &String) -> Option<PaymentRecord> {
    let key = DataKey::Record(id.clone());
    let record = env.storage().persistent().get(&key);
    if record.is_some() {
        extend_persistent(env, &key);
    }
    record
}

pub fn set_contribution(env: &Env, owner: &Address, amount: i128) {
    let key = DataKey::Contribution(owner.clone());
    env.storage().persistent().set(&key, &amount);
    extend_persistent(env, &key);
}

pub fn get_contribution(env: &Env, owner: &Address) -> i128 {
    let key = DataKey::Contribution(owner.clone());
    let amount = env.storage().persistent().get(&key).unwrap_or(0);
    if amount > 0 {
        extend_persistent(env, &key);
    }
    amount
}
