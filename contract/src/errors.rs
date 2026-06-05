use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    InvalidAmount = 1,
    InsufficientEscrow = 2,
    NotFound = 3,
    AlreadyExists = 4,
    InvalidScore = 5,
}
