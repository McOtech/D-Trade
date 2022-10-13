use near_sdk::{AccountId, near_bindgen, borsh::{self, BorshDeserialize, BorshSerialize}, serde::{Serialize, Deserialize}};

use super::{OrderStatus};

#[near_bindgen]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CartItem {
 pub name: String,
 pub serial: String,
 pub price: f64,
 pub quantity: u16,
 pub reference: String,
}

#[near_bindgen]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Cart {
 pub seller: AccountId,
 pub percentage_insurance: u8,
 pub list_for_bidding: bool,
 pub items: Vec<CartItem>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OrderItem {
 pub name: String,
 pub serial: String,
 pub price: u128,
 pub quantity: u16,
 pub reference: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Order {
 pub amount: u128,
 pub seller: AccountId,
 pub status: OrderStatus,
 pub insurance: u8,
 pub delivery: Option<AccountId>,
 pub timestamp: u64,
}

pub struct User {
 pub name: String,
 pub phone: String,
 pub email: String,
}

pub struct Company {
 pub name: String,
 pub phone: String,
 pub email: String,
 pub location: Coordinate,
}

pub struct Coordinate {
 pub lat: i64,
 pub lon: i64,
}

pub struct StarRate {
 pub voters: u64,
 pub votes: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
 pub balance: u128,
 pub total_locked_balance: u128,
 pub account_id: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LockedAmount {
 pub receiver_id: AccountId,
 pub amount: u128,
}