use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, near_bindgen, FunctionError};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub enum OrderStatus {
 PENDING,
 STAGGED,
 SHIPPING,
 DELIVERED,
 CANCELLED
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