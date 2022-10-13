use near_sdk::require;

use crate::*;

#[near_bindgen]
impl Contract {

  pub fn lock_balance(&mut self, account_id: &AccountId, escrow_id: &String, receiver_id: AccountId, lock_amount: u128) {
   require!(lock_amount > 0, "Insufficient funds!");
    let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();

   let mut my_locked_balances = self.locked_balances.get(&id).unwrap_or_else(|| {
       let prefix: Vec<u8> = [
           b"a".as_slice(),
           &near_sdk::env::sha256_array(account_id.as_bytes()),
       ]
       .concat();
       UnorderedMap::new(prefix)
   });

   let locked_amount = LockedAmount {
    receiver_id, amount: lock_amount
   };

   my_locked_balances.insert(&escrow_id, &locked_amount);
  
    let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();
    let mut my_balance = self.balances.get(&id).unwrap_or_else(|| {
      Account {
        balance: 0,
        total_locked_balance: 0,
        account_id: account_id.clone()
      }
    });
    
    my_balance.total_locked_balance += lock_amount;
    self.balances.insert(&id, &my_balance);
  }

  pub fn get_locked_balance(&self, account_id: AccountId, escrow_id: String,  receiver_id: AccountId) -> LockedAmount {
    let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();

   if let Some(my_locked_balances) = self.locked_balances.get(&id) {
    let locked_balance = my_locked_balances.get(&escrow_id).unwrap_or(LockedAmount { receiver_id, amount: 0 });
    locked_balance
   } else {
    LockedAmount {
     receiver_id, amount: 0
    }
   }
  }

  pub fn refund(&mut self, account_id: AccountId, escrow_id: String) -> AccountId {
    let id: String = String::from_utf8(near_sdk::env::sha256(account_id.as_bytes()).to_vec()).unwrap();

    let mut my_locked_balances = self.locked_balances.get(&id).unwrap_or_else(|| {
      env::panic_str("No records found!");
    });
    
    let locked_balance = my_locked_balances.get(&escrow_id).unwrap_or_else(|| {
      env::panic_str("Invalid Order ID!");
    });
    let refund_amount = locked_balance.amount;
    let receiver_id = locked_balance.receiver_id;

    let mut my_balance = self.balances.get(&id).unwrap_or_else(|| {
      Account {
        balance: 0,
        total_locked_balance: 0,
        account_id
      }
    });

    my_balance.balance += refund_amount;
    my_balance.total_locked_balance -= refund_amount;
    self.balances.insert(&id, &my_balance);
    my_locked_balances.remove(&escrow_id);
    receiver_id
  }

}
