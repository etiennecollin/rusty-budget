use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

/// Represents a currency symbol.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CurrencySymbol {
    CAD,
    USD,
    EUR,
}

/// Represents an amount of money in cents.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Amount {
    amount_cents: i64,
    currency_symbol: CurrencySymbol,
}

/// Implements the Amount struct.
impl Amount {
    /// Creates a new amount with the given amount in cents and currency symbol.
    pub fn new(amount_cents: i64, currency_symbol: CurrencySymbol) -> Amount {
        Amount {
            amount_cents: amount_cents,
            currency_symbol: currency_symbol,
        }
    }
    /// Returns the amount in cents.
    pub fn get_amount_cents(&self) -> i64 {
        self.amount_cents
    }
    /// Sets the amount in cents.
    pub fn set_amount_cents(&mut self, amount_cents: i64) {
        self.amount_cents = amount_cents;
    }

    /// Returns the currency symbol of the amount.
    pub fn get_currency_symbol(&self) -> CurrencySymbol {
        self.currency_symbol
    }

    /// Sets the currency symbol of the amount.
    pub fn set_currency_symbol(&mut self, currency_symbol: CurrencySymbol) {
        self.currency_symbol = currency_symbol;
    }

    /// Returns a string representation of the cents as dollars with the format: 'X.XX'.
    pub fn get_amount_display(&self) -> String {
        let is_negative: bool = self.amount_cents < 0;
        let mut amount: String = self.get_amount_cents().abs().to_string();
        let length: usize = amount.len();

        match length {
            1 => amount.insert_str(0, "0.0"),
            2 => amount.insert_str(0, "0."),
            _ => {
                amount.insert(length - 2, '.');
            }
        }

        if is_negative {
            amount.insert(0, '-');
        }

        amount
    }
}

/// Implements the PartialEq trait for Amount.
/// Two amounts are equal if they have the same amount in cents and currency symbol.
impl PartialEq for Amount {
    /// Returns true if the amounts in cents and the currencies are equal.
    fn eq(&self, other: &Self) -> bool {
        (self.get_amount_cents() == other.get_amount_cents())
            && (self.get_currency_symbol() == other.get_currency_symbol())
    }
}

/// Implements the Eq trait for Amount.
impl Eq for Amount {}

/// Implements the Add trait for Amount.
/// Two amounts can be added if they have the same currency symbol.
/// The result is a new amount with the same currency symbol and the sum of the amounts in cents.
impl Add for Amount {
    type Output = Amount;

    /// Adds the other amount from this amount.
    /// Returns a new amount with the same currency symbol and the sum of the amounts in cents.
    ///
    /// # Panics
    /// Panics if the currency symbols are different.
    ///
    /// # Examples
    /// ```rust
    /// use rusty_budget::util::amount::{Amount, CurrencySymbol};
    /// let amount1 = Amount::new(100, CurrencySymbol::CAD);
    /// let amount2 = Amount::new(50, CurrencySymbol::CAD);
    /// let amount3 = amount1 + amount2;
    /// assert_eq!(amount3, Amount::new(150, CurrencySymbol::CAD));
    /// ```
    fn add(self, other: Amount) -> Amount {
        if self.currency_symbol != other.currency_symbol {
            panic!(
                "Cannot add amounts with different currency symbols. {:?} != {:?}",
                self.currency_symbol, other.currency_symbol
            )
        }

        let amount_cents = self.amount_cents + other.amount_cents;
        let currency_symbol = self.currency_symbol;

        Amount::new(amount_cents, currency_symbol)
    }
}

/// Implements the Sub trait for Amount.
/// Two amounts can be subtracted if they have the same currency symbol.
/// The result is a new amount with the same currency symbol and the difference of the amounts in cents.
impl Sub for Amount {
    type Output = Amount;

    /// Subtracts the other amount from this amount.
    /// Returns a new amount with the same currency symbol and the difference of the amounts in cents.
    ///
    /// # Panics
    /// Panics if the currency symbols are different.
    ///
    /// # Examples
    /// ```rust
    /// use rusty_budget::util::amount::{Amount, CurrencySymbol};
    /// let amount1 = Amount::new(100, CurrencySymbol::CAD);
    /// let amount2 = Amount::new(50, CurrencySymbol::CAD);
    /// let amount3 = amount1 - amount2;
    /// assert_eq!(amount3, Amount::new(50, CurrencySymbol::CAD));
    /// ```
    fn sub(self, other: Amount) -> Amount {
        if self.currency_symbol != other.currency_symbol {
            panic!(
                "Cannot add amounts with different currency symbols. {:?} != {:?}",
                self.currency_symbol, other.currency_symbol
            )
        }

        let amount_cents = self.amount_cents - other.amount_cents;
        let currency_symbol = self.currency_symbol;

        Amount::new(amount_cents, currency_symbol)
    }
}
