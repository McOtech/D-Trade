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
use utils::{Account, LockedAmount, OrderItem, Order, UserOrder, OrderBundle, OrderStatus, User, CourierUser, Vehicle, Courier, Company, CourierProfile, Proposal, ProposalStatus, CourierClientView};


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    token_precision: u8,
    balances: LookupMap<String, Account>,
    locked_balances: LookupMap<String, UnorderedMap<String, LockedAmount>>,
    orders: LookupMap<String, UnorderedMap<String, (Order, Vector<OrderItem>, Vector<String>)>>,
    orders_pending: LookupMap<String, UnorderedMap<String, String>>,
    orders_staged: LookupMap<String, UnorderedMap<String, String>>,
    orders_shipping: LookupMap<String, UnorderedMap<String, String>>,
    couriers: LookupMap<String, User>,
    couriers_by_company: LookupMap<String, UnorderedMap<String, u64>>,
    courier_companies: LookupMap<String, Vector<String>>,
    companies: LookupMap<String, Company>,
    proposals: LookupMap<String, UnorderedMap<String, Proposal>> // courier -> order_id -> proposal
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            token_precision: 18,
            balances: LookupMap::new(b"b"), // b
            locked_balances: LookupMap::new(b"l"), // l, a
            orders: LookupMap::new(b"o"), // o, n, p, s
            orders_pending: LookupMap::new(b"c"), // c, f
            orders_staged: LookupMap::new(b"d"), // d
            orders_shipping: LookupMap::new(b"e"), // e
            couriers: LookupMap::new(b"g"), // g
            couriers_by_company: LookupMap::new(b"h"), // h
            companies: LookupMap::new(b"i"), // i, j
            courier_companies: LookupMap::new(b"k"), // k, m
            proposals: LookupMap::new(b"q") // q, r
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> String {
        let raw_msg = msg.split("|").collect::<Vec<&str>>();
        let txn_type = raw_msg[0];
        let txn = raw_msg[1];
        match txn_type {
            "place_order" => {
                if let Some(balance) = self.place_order(sender_id, amount.0, txn.to_string(), self.token_precision) {
                    return balance.to_string()
                } else {
                    return amount.0.to_string()
                }
            },
            "approve_proposal" => {
                if let Some(balance) = self.approve_proposal(sender_id, amount.0, txn.to_string()) {
                    return balance.to_string()
                } else {
                    return amount.0.to_string()
                }
            },
            // "ship" => {},
            _ => {
                return amount.0.to_string()
            }
        }
    }

    pub fn get_buyer_orders(&self, page: u16, limit: u16) -> Option<OrderBundle> {
        let account_id = env::predecessor_account_id();
        let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();
        if let Some(my_orders) = self.orders.get(&id) {
            let mut orders: Vec<UserOrder> = vec![];
            my_orders.keys().skip(page.into()).take(limit.into()).for_each(|key| {
                let persisted_order = my_orders.get(&key).unwrap();
                let _products = persisted_order.1.iter().collect::<Vec<OrderItem>>();
                let user_order = UserOrder {
                    id: key,
                    metadata: persisted_order.0,
                    products: _products,
                };
                orders.push(user_order);
            });
            let next_page = page + limit;
            let bundled_order = OrderBundle {
                next_page,
                orders
            };
            return Some(bundled_order);
        }
        None
    }

    pub fn get_shipping_suggestions(&self, buyer_id: String, order_id: String, page: u16, limit: u16) -> Vec<CourierClientView> {
        let mut suggestions: Vec<CourierClientView> = vec![];
        if let Some(order) = self.orders.get(&buyer_id).unwrap().get(&order_id) {
            order.2.iter().skip(page.into()).take(limit.into()).for_each(|courier_id| {
                if let Some(proposal) = self.proposals.get(&courier_id).unwrap().get(&order_id) {
                    let courier = self.couriers.get(&courier_id).unwrap();
                    let profile = courier.courier_profile.unwrap();
                    let courier_client_view = CourierClientView {
                        id: courier_id,
                        name: courier.name,
                        image: courier.image,
                        phone: courier.phone,
                        on_transit: profile.on_transit,
                        feedback: profile.feedback,
                        proposed_fee: proposal.fee,
                    };
                    suggestions.push(courier_client_view)
                }
            });
        }
        return suggestions
    }

    pub fn clear_order_couriers(&self, order_id: String, limit: u16) -> Vec<String> {
        let account_id = env::predecessor_account_id();
        let id = self.get_hash(account_id);
        let mut order = self.orders.get(&id).unwrap().get(&order_id).unwrap();
        
        let approved_courier = order.0.courier.unwrap();
        let mut page = order.2.len().min(limit as u64);
        let mut courier_ids: Vec<String> = vec![];
        
        while page > 0 {
            let len = order.2.len();
            let courier_id = order.2.get(len - 1).unwrap();
            if courier_id != approved_courier {
                if self.proposals.get(&courier_id).unwrap().remove(&order_id).is_some() {
                    if let Some(removed_id) = order.2.pop() {
                        courier_ids.push(removed_id);
                    }
                }
            } else {
                order.2.pop();
            }
            page -= 1;
        }

        courier_ids
    }

    /*pub fn get_order(&self, order_id: String, account_id: Option<String>) -> Option<UserOrder> {
        if let Some(id) = account_id {
            return self.retrieve_order(id, order_id)
        }
        let signer = env::predecessor_account_id();
        let id: String = String::from_utf8(near_sdk::env::sha256(signer.as_bytes()).to_vec()).unwrap();
        return self.retrieve_order(id, order_id)
    } */

    pub fn get_pending_orders(&self, page: u16, limit: u16) -> Option<OrderBundle> {
        let account_id = env::predecessor_account_id();
        let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();
        if let Some(pending_orders) = self.orders_pending.get(&id) {
            let mut orders: Vec<UserOrder> = vec![];
            pending_orders.keys().skip(page.into()).take(limit.into()).for_each(|order_id| {
                let buyer_id = pending_orders.get(&order_id).unwrap();
                if let Some(user_order) = self.retrieve_order(buyer_id, order_id) {
                    orders.push(user_order);
                }
            });
            let next_page = page + limit;
            let bundled_order = OrderBundle {
                next_page,
                orders
            };
            return Some(bundled_order)
        }
        None
    }

    pub fn get_staged_orders(&self, page: u16, limit: u16/*, account_id: Option<String> */) -> Option<OrderBundle> {
        /* if let Some(id) = account_id {
            return self.retrieve_staged_order(id, page, limit)
        } */
        let signer = env::predecessor_account_id();
        let id: String = String::from_utf8(near_sdk::env::sha256(signer.as_bytes()).to_vec()).unwrap();
        return self.retrieve_staged_order(id, page, limit)
    }

    pub fn stage_order(&self, order_id: String) -> Option<String> {
        let account_id = env::predecessor_account_id();
        let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();
        if let Some(pending_orders) = self.orders_pending.get(&id) {
            if let Some(buyer_id) = pending_orders.get(&order_id) {
                if let Some(_order) = self.orders.get(&buyer_id).unwrap().get(&order_id) {
                    let mut order = _order.0;
                    order.status = OrderStatus::STAGGED;
                    let updated_order = (order, _order.1, _order.2);
                    self.orders.get(&buyer_id).unwrap().insert(&order_id, &updated_order);
                    return Some(order_id)
                }
            }
        }
        None
    }

    pub fn register_courier(&mut self, profile: CourierUser) {
        let account_id = env::predecessor_account_id();
        let vehicle = match profile.vehicle.as_str() {
            "motorcycle" => Vehicle::MOTORCYCLE,
            "tuktuk" => Vehicle::TUKTUK,
            "car" => Vehicle::CAR,
            "pickup" => Vehicle::PICKUP,
            "lorry" => Vehicle::LORRY,
            _ => env::panic_str("Invalid vehicle record!")
        };

        let courier = Courier {
            vehicle,
            make_model: profile.make_model,
            plate_id: profile.plate_id,
            on_transit: false,
            feedback: None,
        };
        let user = User {
            name: profile.name,
            phone: profile.phone,
            email: profile.email,
            image: profile.image,
            courier_profile: Some(courier)
        };
        let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();
        self.couriers.insert(&id, &user);
    }

    pub fn save_company(&mut self, company_id: String) {
        let account_id = env::predecessor_account_id();
        let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();
        if let Some(_company) = self.companies.get(&company_id) {
            let mut company_couriers = self.couriers_by_company.get(&company_id).unwrap_or_else(|| {
                let prefix: Vec<u8> = [
                        b"j".as_slice(),
                        &near_sdk::env::sha256_array(company_id.as_bytes()),
                    ]
                    .concat();
                UnorderedMap::new(prefix)
            });
            company_couriers.insert(&id, &0);
            let mut courier_companies = self.courier_companies.get(&id).unwrap_or_else(|| {
                let prefix: Vec<u8> = [
                        b"m".as_slice(),
                        &near_sdk::env::sha256_array(id.as_bytes()),
                    ]
                    .concat();
                Vector::new(prefix)
            });
            courier_companies.push(&company_id);
        } else {
            env::panic_str("Company does not exist!");
        }
    }

    pub fn courier_saved_companies(&self, page: u8, limit: u8) -> Option<Vec<Company>> {
        let account_id = env::predecessor_account_id();
        let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();
        let mut companies: Vec<Company> = vec![];
        if let Some(list_of_companies) = self.courier_companies.get(&id) {
            list_of_companies.iter().skip(page.into()).take(limit.into()).for_each(|company_id| {
                if let Some(company) = self.companies.get(&company_id) {
                    companies.push(company);
                }
            });
        }
        Some(companies)
    }

    pub fn company_couriers(&self, company_id: String, page: u8, limit: u8) -> Option<Vec<CourierProfile>> {
        let mut couriers: Vec<CourierProfile> = vec![];
        if let Some(_couriers) = self.couriers_by_company.get(&company_id) {
            _couriers.keys().skip(page.into()).take(limit.into()).for_each(|courier_id| {
                if let Some(number_of_deliveries) = _couriers.get(&courier_id) {
                    let courier = self.couriers.get(&courier_id).unwrap();
                    let courier_profile = CourierProfile {
                        deliveries: number_of_deliveries,
                        profile: courier
                    };
                    couriers.push(courier_profile);
                }
            });
        }
        Some(couriers)
    }

    pub fn place_proposal(&self, courier_id: String, order_id: String) {
        let account_id = env::predecessor_account_id();
        let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();
        let proposal = Proposal {
            client: id.clone(),
            courier_id: None,
            amount: 0,
            fee: 0,
            status: ProposalStatus::PENDING
        };
        let mut courier_proposals = self.proposals.get(&courier_id).unwrap_or_else(|| {
            let prefix: Vec<u8> = [
                    b"r".as_slice(),
                    &near_sdk::env::sha256_array(courier_id.as_bytes()),
                ]
                .concat();
            UnorderedMap::new(prefix)
        });
        courier_proposals.insert(&order_id, &proposal);
        let mut order = self.orders.get(&id).unwrap().get(&order_id).unwrap();
        order.2.push(&courier_id);
    }

    pub fn get_proposals(&self, page: u16, limit: u16) -> Option<OrderBundle> {
        let account_id = env::predecessor_account_id();
        let id = self.get_hash(account_id);
        if let Some(order_proposals) = self.proposals.get(&id) {
            let mut orders: Vec<UserOrder> = vec![];
            order_proposals.keys().skip(page.into()).take(limit.into()).for_each(|order_id| {
                let proposal = order_proposals.get(&order_id).unwrap();
                if let Some(user_order) = self.retrieve_order(proposal.client, order_id) {
                    orders.push(user_order);
                }
            });
            let next_page = page + limit;
            let order_bundle = OrderBundle {
                next_page,
                orders
            };
            return Some(order_bundle)
        }
        None
    }

    pub fn suggest_shipping_fee(&self, order_id: String, amount: u128) {
        let account_id = env::predecessor_account_id();
        let id = self.get_hash(account_id);
        if let Some(proposal) = self.proposals.get(&id).unwrap().get(&order_id) {
            let new_proposal = Proposal {
                fee: amount,
                ..proposal
            };
            self.proposals.get(&id).unwrap().insert(&order_id, &new_proposal);
        }
    }

    fn get_hash(&self, value: AccountId) -> String {
        let hash: String = String::from_utf8(near_sdk::env::sha256(value.as_bytes()).to_vec()).unwrap();
        hash
    }

    fn retrieve_order(&self, id: String, order_id: String) -> Option<UserOrder> {
        if let Some(order) = self.orders.get(&id).unwrap().get(&order_id) {
            let _products = order.1.iter().collect::<Vec<OrderItem>>();
            let user_order = UserOrder {
                id: order_id,
                metadata: order.0,
                products: _products,
            };
            return Some(user_order)
        }
        return None
    }

    fn retrieve_staged_order(&self, id: String, page: u16, limit: u16) -> Option<OrderBundle> {
        if let Some(stagged_orders) = self.orders_staged.get(&id) {
            let mut orders: Vec<UserOrder> = vec![];
            stagged_orders.keys().skip(page.into()).take(limit.into()).for_each(|order_id| {
                let buyer_id = stagged_orders.get(&order_id).unwrap();
                if let Some(user_order) = self.retrieve_order(buyer_id, order_id) {
                    orders.push(user_order);
                }
            });
            let next_page = page + limit;
            let bundled_order = OrderBundle {
                next_page,
                orders
            };
            return Some(bundled_order)
        }
        None
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    
}
