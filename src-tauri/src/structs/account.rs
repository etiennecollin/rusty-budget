use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};
use crate::structs::*;

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);


/// Represents a bank account.
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Account {
    description: String,
    id: usize,
    balance: Amount,
    transactions: Vec<Transaction>,
}

/// Implements the Account struct.
impl Account {
    /// Creates a new account with the given description and currency symbol.
    pub fn new(description: String, currency_symbol: CurrencySymbol) -> Self {
        Account {
            description: description,
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            balance: Amount::new(0, currency_symbol),
            transactions: Vec::new(),
        }
    }
    /// Returns the description of the account.
    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    /// Sets the description of the account.
    pub fn set_description(&mut self, description: String) {
        match description.as_str() {
            "" => panic!("Description cannot be empty"),
            _ => self.description = description,
        }
    }

    /// Returns the id of the account.
    pub fn get_id(&self) -> usize {
        self.id
    }

    /// Returns the balance of the account.
    pub fn get_balance(&self) -> Amount {
        self.balance.clone()
    }

    /// Sets the balance of the account.
    pub fn set_balance(&mut self, balance: Amount) {
        self.balance = balance;
    }

    /// Returns the transactions of the account.
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    /// Sets the transactions of the account.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        let new_balance = self.get_balance() + transaction.get_amount();
        self.set_balance(new_balance);

        self.transactions.push(transaction);
    }

    /// Deletes the given transaction from the account.
    pub fn delete_transaction(&mut self, transaction: Transaction) -> Option<Transaction> {
        let index = self.transactions.iter().position(|x| *x == transaction);

        match index {
            Some(index) => {
                let transaction = self.transactions[index].clone();

                let new_balance = self.get_balance() - transaction.get_amount();
                self.set_balance(new_balance);

                self.transactions.remove(index);
                Some(transaction)
            }
            None => None,
        }
    }
}

/// Implements the PartialEq trait for Account.
/// Two accounts are equal if they have the same id.
impl PartialEq for Account {
    /// Returns true if the id of the account is equal to the id of the other account.
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

/// Implements the Eq trait for Account.
impl Eq for Account {}
