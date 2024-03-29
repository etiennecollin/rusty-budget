use crate::structs::*;
use crate::utils::*;
use serde::{Deserialize, Serialize};

/// Represents a transaction.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    kind: TransactionKind,
    description: String,
    id: u128,
    date: String,
    amount: Amount,
}

/// Implements the Transaction struct.
impl Transaction {
    /// Creates a new transaction with the given transaction type, description, date, and amount.
    /// The date must be in the format: 'YYYY-MM-DD' and be valid.
    /// For example: '2021-11-30'.
    /// # Panics
    /// Panics if the date is invalid or improperly formatted.
    pub fn new(
        kind: TransactionKind,
        description: String,
        date: String,
        amount: Amount,
    ) -> Self {
        Transaction {
            kind,
            description: description,
            id: generate_id(),
            date: verify_date_format(date),
            amount: amount,
        }
    }

    /// Returns the type of the transaction.
    pub fn get_kind(&self) -> TransactionKind {
        self.kind.clone()
    }

    /// Sets the type of the transaction.
    pub fn set_kind(&mut self, kind: TransactionKind) {
        self.kind = kind
    }

    /// Returns the description of the transaction.
    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    /// Sets the description of the transaction.
    /// The description cannot be empty.
    /// # Panics
    /// Panics if the description is empty.
    pub fn set_description(&mut self, description: String) {
        match description.as_str() {
            "" => panic!("Description cannot be empty"),
            _ => self.description = description,
        }
    }

    /// Returns the date of the transaction.
    pub fn get_date(&self) -> String {
        self.date.clone()
    }

    /// Sets the date of the transaction.
    /// The date must be in the format: 'YYYY-MM-DD' and be valid.
    /// For example: '2021-11-30'.
    /// # Panics
    /// Panics if the date is invalid or improperly formatted.
    pub fn set_date(&mut self, date_string: String) {
        self.date = verify_date_format(date_string);
    }

    /// Returns the id of the transaction.
    pub fn get_id(&self) -> u128 {
        self.id
    }

    /// Returns the amount of the transaction.
    pub fn get_amount(&self) -> Amount {
        self.amount.clone()
    }

    /// Sets the amount of the transaction.
    pub fn set_amount(&mut self, amount: Amount) {
        self.amount = amount;
    }
}

/// Implements the PartialEq trait for the Transaction struct.
/// Two transactions are equal if they have the same id.
impl PartialEq for Transaction {
    /// Returns true if the transactions have the same id.
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

/// Implements the Eq trait for the Transaction struct.
impl Eq for Transaction {}
