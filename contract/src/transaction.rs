
use std::ops::{Mul, Sub};

use near_sdk::{serde_json};

use crate::{*, utils::{OrderItem, Cart, CartItem, ProposalApproval, Coordinate}};

#[near_bindgen]
impl Contract {
  pub fn approve_proposal(&self, account_id: AccountId, amount: u128, payload: String) -> Option<u128> {
    let proposal_approval: ProposalApproval = match serde_json::from_str(payload.as_str()) {
      Ok(_proposal) => _proposal,
      Err(_error) => {
        return None
      }
    };
    let buyer_id = self.get_hash(account_id);
    if let Some(proposals) = self.proposals.get(&proposal_approval.courier_id) {
      if let Some(proposal) = proposals.get(&proposal_approval.order_id) {
        let expected_amount = proposal.fee.mul(2);
        if amount.ge(&expected_amount) {
          let balance = amount.sub(expected_amount);
          let new_proposal = Proposal {
            amount: expected_amount,
            status: ProposalStatus::APPROVED,
            ..proposal
          };
          // update order
          if let Some(_order) = self.orders.get(&buyer_id) {
            if let Some(order) = _order.get(&proposal_approval.order_id) {
              let new_order = Order {
                courier: Some(proposal_approval.courier_id.clone()),
                ..order.0
              };
              self.orders.get(&buyer_id).unwrap().insert(&proposal_approval.order_id, &(new_order, order.1, order.2));
              // update proposal
              self.proposals.get(&proposal_approval.courier_id).unwrap().insert(&proposal_approval.order_id, &new_proposal);
              return Some(balance)
            }
          }
        }
      }
    }
    None
  }

    // #[handle_result]
 pub fn place_order(&mut self, account_id: AccountId, amount_paid: u128, products_ordered: String, decimal: u8) -> Option<u128> /*Result<u128, InternalError>*/ {
    // Deceserialize cart object
    let cart: Cart = match serde_json::from_str(products_ordered.as_str()) {
      Ok(_cart) => _cart,
      Err(_error) => {
        return None // Err(InternalError::Unexpected(error.to_string()))
      }
    };
    
    let date = env::block_timestamp_ms();
    // Get order id
    let order_id = match String::from_utf8(near_sdk::env::sha256(date.to_string().as_bytes()).to_vec()) {
      Ok(_order_id) => _order_id,
      Err(_error) => {
        return None // Err(InternalError::Unexpected(error.to_string()))
      }
    };

    if let Some((amount, items_store, couriers)) = self.process_ordered_items(&order_id, cart.items, decimal) {
      if let Some(balance) = self.register_order(&account_id, date, amount, items_store, couriers, amount_paid, &order_id, cart.seller, cart.list_for_bidding, cart.percentage_insurance, cart.location) {
        return Some(balance) // Ok(balance)
      }
      return None // Err(InternalError::Unexpected("Error placing order!".to_string()))
    }
    return None // Err(InternalError::Unexpected("Error processing ordered items!".to_string()))
 }

  fn register_order(&mut self, account_id: &AccountId, date: u64, total_cost: u128, items_store: Vector<OrderItem>, couriers: Vector<String>, amount_paid: u128, order_id: &String, seller: String, list_for_bidding: bool, percentage_insurance: u8, location: Coordinate) -> Option<u128> {
    // Get user id
    let id: String = match String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()) {
      Ok(_id) => _id,
      Err(_error) => {
        return None // Err(InternalError::Unexpected(error.to_string()))
      }
    };

    let mut my_orders = self.orders.get(&id).unwrap_or_else(|| {
        let prefix: Vec<u8> = [
            b"n".as_slice(),
            &near_sdk::env::sha256_array(account_id.as_bytes()),
        ]
        .concat();
        UnorderedMap::new(prefix)
    });

    let courier = if list_for_bidding == true { None } else { Some(id.clone()) };
    let order = Order {
        amount: total_cost,
        seller: seller.clone(),
        status: utils::OrderStatus::PENDING,
        insurance: percentage_insurance,
        timestamp: date,
        courier,
        location,
    };

    let value = (order, items_store, couriers);
    if amount_paid < total_cost {
      return None // Err(InternalError::Unexpected("Insufficient funds!".to_string()))
    }
    if my_orders.insert(order_id, &value).is_none() {
      if let Some(_res) = self.update_pending_orders(&seller, order_id, &id) {
        if let Some(company) = self.companies.get(&seller) {
          // Check if enough money has been paid
          self.lock_balance(account_id, order_id, company.wallet, total_cost);
          let bal = amount_paid - total_cost;
          return Some(bal) // Ok(bal)
        }
      }
    }
    return None // Err(InternalError::Unexpected("Order exists!".to_string()))
  }

  fn process_ordered_items(&self, order_id: &String, cart_items: Vec<CartItem>, decimal: u8) -> Option<(u128, Vector<OrderItem>, Vector<String>)> {
    let mut amount: u128 = 0;
    let prefix: Vec<u8> = [
            b"p".as_slice(),
            &near_sdk::env::sha256_array(order_id.as_bytes()),
        ]
        .concat();

    let mut items_store = Vector::new(prefix);

    cart_items.into_iter().for_each(|item| {
    // convert price to u128
    let price = (item.price * 10_f64.powi(decimal.into())) as u128;
    // create OrderItem object
    let order_item = OrderItem {
      price,
      name: item.name,
      serial: item.serial,
      quantity: item.quantity,
      reference: item.reference
    };
    items_store.push(&order_item);
    // Add price to total amount
    amount = amount.checked_add(price).unwrap_or(amount);
    });
    let courier_prefix: Vec<u8> = [
            b"s".as_slice(),
            &near_sdk::env::sha256_array(order_id.as_bytes()),
        ]
        .concat();

    let couriers = Vector::new(courier_prefix);
    Some((amount, items_store, couriers))
  }

  fn update_pending_orders(&self, seller_id: &String, order_id: &String, user_id: &String) -> Option<String> {
    let mut seller_pending_orders = self.orders_pending.get(seller_id).unwrap_or_else(|| {
        let prefix: Vec<u8> = [
            b"f".as_slice(),
            &near_sdk::env::sha256_array(seller_id.as_bytes()),
        ]
        .concat();
        UnorderedMap::new(prefix)
    });
    seller_pending_orders.insert(order_id, user_id);
    Some("".to_string())
  }
}