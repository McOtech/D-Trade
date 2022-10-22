use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, near_bindgen, FunctionError, serde::{Serialize, Deserialize}};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum OrderStatus {
 PENDING,
 STAGGED,
 SHIPPING,
 DELIVERED,
 CANCELLED
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Vehicle {
    MOTORCYCLE,
    TUKTUK,
    CAR,
    PICKUP,
    LORRY
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub enum ProposalStatus {
    PENDING,
    PROPOSED,
    APPROVED
}

pub enum InternalError {
    NotFound,
    Unexpected(String),
}

impl FunctionError for InternalError {
    fn panic(&self) -> ! {
        match self {
            InternalError::NotFound => 
                near_sdk::env::panic_str("not found"),
            InternalError::Unexpected(message) => 
                near_sdk::env::panic_str(&format!("unexpected error: {}", message))
        }
    }
}