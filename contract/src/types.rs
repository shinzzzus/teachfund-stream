use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentRecord {
    pub id: String,
    pub owner: Address,
    pub target: i128,
    pub funded: i128,
    pub score: u32,
    pub released: i128,
    pub status: String,
    pub updated_ledger: u32,
}
