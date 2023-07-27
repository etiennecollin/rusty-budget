use serde::{Deserialize, Serialize};

use crate::structs::*;
use crate::utils::*;

/// Represents a user file.
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct File {
    name: String,
    id: u128,
    bank_accounts: Vec<Account>,
}

/// Implements the Person struct.
impl File {
    /// Creates a new person with the given name.
    pub fn new(name: String) -> Self {
        File {
            name: name,
            id: generate_id(),
            bank_accounts: Vec::new(),
        }
    }

    /// Returns the name of the person.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Sets the name of the person.
    pub fn set_name(&mut self, name: String) {
        match name.as_str() {
            "" => panic!("Name cannot be empty"),
            _ => self.name = name,
        }
    }

    /// Returns the id of the person.
    pub fn get_id(&self) -> u128 {
        self.id
    }

    /// Returns the bank accounts of the person.
    pub fn get_accounts(&self) -> Vec<Account> {
        self.bank_accounts.clone()
    }

    /// Sets the bank accounts of the person.
    pub fn set_accounts(&mut self, bank_accounts: Vec<Account>) {
        self.bank_accounts = bank_accounts;
    }

    /// Adds a bank account to the person.
    pub fn add_account(&mut self, account: Account) {
        self.bank_accounts.push(account);
    }

    /// Deletes a bank account from the person.
    pub fn delete_account(&mut self, account: Account) -> Option<Account> {
        let index = self.bank_accounts.iter().position(|x| *x == account);

        match index {
            Some(index) => {
                let account = self.bank_accounts[index].clone();
                self.bank_accounts.remove(index);
                Some(account)
            }
            None => None,
        }
    }
}

/// Implements the PartialEq trait for the Person struct.
/// Two persons are equal if they have the same id.
impl PartialEq for File {
    /// Returns true if the persons have the same id.
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

/// Implements the Eq trait for the Person struct.
impl Eq for File {}
