use soroban_sdk::{contractevent, Address, String};

#[contractevent(topics = ["created"])]
pub struct CreatedEvent {
    pub id: String,
    pub owner: Address,
    pub target: i128,
}

#[contractevent(topics = ["funded"])]
pub struct FundedEvent {
    pub id: String,
    pub from: Address,
    pub amount: i128,
}

#[contractevent(topics = ["attested"])]
pub struct AttestedEvent {
    pub id: String,
    pub score: u32,
}

#[contractevent(topics = ["released"])]
pub struct ReleasedEvent {
    pub id: String,
    pub to: Address,
    pub amount: i128,
}
