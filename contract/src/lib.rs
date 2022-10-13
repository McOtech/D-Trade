/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

mod escrow;
mod transaction;
mod utils;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::json_types::U128;
use utils::{Account, LockedAmount, OrderItem, Order};


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    token_precision: u8,
    balances: LookupMap<String, Account>,
    locked_balances: LookupMap<String, UnorderedMap<String, LockedAmount>>,
    orders: LookupMap<String, UnorderedMap<String, (Order, Vector<OrderItem>)>>,
    orders_pending: LookupMap<String, UnorderedMap<String, String>>,
    orders_stagged: LookupMap<String, UnorderedMap<String, String>>,
    orders_shipping: LookupMap<String, UnorderedMap<String, String>>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            token_precision: 18,
            balances: LookupMap::new(b"b"), // b
            locked_balances: LookupMap::new(b"l"), // l, a
            orders: LookupMap::new(b"o"), // o, n, p
            orders_pending: LookupMap::new(b"c"), // c, f
            orders_stagged: LookupMap::new(b"d"), // d
            orders_shipping: LookupMap::new(b"e"), // e
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> String {
        if let Some(balance) = self.place_order(sender_id, amount.0, msg, self.token_precision) {
            balance.to_string()
        } else {
            amount.0.to_string()
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    
}
