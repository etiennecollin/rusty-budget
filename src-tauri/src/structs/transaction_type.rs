use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

/// TransactionType is an enum that represents the type of transaction.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum TransactionType {
    Income(IncomeCategory),
    Expense(ExpenseCategory),
}

/// Implements the TransactionType struct.
impl TransactionType {
    /// Returns the type of the transaction as a string.
    pub fn get_type_as_string(&self) -> String {
        match self {
            TransactionType::Income(_) => "Income".to_owned(),
            TransactionType::Expense(_) => "Expense".to_owned(),
        }
    }

    /// Returns the category of the transaction as a string no matter its type.
    pub fn get_category_as_string(&self) -> String {
        match self {
            TransactionType::Income(category) => format!("{:?}", category).to_owned(),
            TransactionType::Expense(category) => format!("{:?}", category).to_owned(),
        }
    }
}

/// IncomeCategory is an enum that represents the category of income.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, EnumIter, EnumString)]
pub enum IncomeCategory {
    Salary,
    Gift,
    Other,
}

/// ExpenseCategory is an enum that represents the category of expense.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, EnumIter, EnumString)]
pub enum ExpenseCategory {
    Housing,
    Utilities,
    Insurance,
    MedicalHealthcare,
    SavingInvesting,
    Groceries,
    School,
    Transportation,
    PersonalSpending,
    RecreationEntertainment,
    Miscellaneous,
}
