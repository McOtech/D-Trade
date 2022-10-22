use near_sdk::{AccountId, near_bindgen, borsh::{self, BorshDeserialize, BorshSerialize}, serde::{Serialize, Deserialize}};

use super::{OrderStatus, Vehicle, ProposalStatus};

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
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Cart {
 pub seller: String,
 pub location: Coordinate,
 pub percentage_insurance: u8,
 pub list_for_bidding: bool,
 pub items: Vec<CartItem>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OrderItem {
 pub name: String,
 pub serial: String,
 pub price: u128,
 pub quantity: u16,
 pub reference: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Order {
 pub amount: u128,
 pub seller: String,
 pub status: OrderStatus,
 pub insurance: u8,
 pub courier: Option<String>,
 pub timestamp: u64,
 pub location: Coordinate,
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserOrder {
 pub id: String,
 pub metadata: Order,
 pub products: Vec<OrderItem>
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OrderBundle {
 pub next_page: u16,
 pub orders: Vec<UserOrder>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
 pub name: String,
 pub phone: String,
 pub email: String,
 pub image: String,
 pub courier_profile: Option<Courier>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Courier {
 pub vehicle: Vehicle,
 pub make_model: String,
 pub plate_id: String,
 pub on_transit: bool,
 pub feedback: Option<Feedback>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Feedback {
 pub score: u64,
 pub star_rate: StarRate,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StarRate {
 pub voters: u64,
 pub votes: u64,
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CourierUser {
 pub name: String,
 pub phone: String,
 pub email: String,
 pub image: String,
 pub vehicle: String,
 pub make_model: String,
 pub plate_id: String,
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CourierClientView {
 pub id: String,
 pub name: String,
 pub image: String,
 pub phone: String,
 pub on_transit: bool,
 pub feedback: Option<Feedback>,
 pub proposed_fee: u128,
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalApproval {
 pub order_id: String,
 pub courier_id: String,
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CourierProfile {
 pub deliveries: u64,
 pub profile: User,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Company {
 pub name: String,
 pub wallet: AccountId,
 pub phone: String,
 pub email: String,
 pub location: Coordinate,
 pub sales: u64,
 pub star_rate: StarRate,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Coordinate {
 pub lat: i64,
 pub lon: i64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposal {
 pub client: String,
 pub courier_id: Option<AccountId>,
 pub amount: u128,
 pub fee: u128,
 pub status: ProposalStatus
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