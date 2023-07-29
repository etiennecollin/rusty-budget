use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

/// TransactionType is an enum that represents the type of transaction.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum TransactionKind {
    Income(IncomeCategory),
    Expense(ExpenseCategory),
}

/// Implements the TransactionType struct.
impl TransactionKind {
    /// Returns the type of the transaction as a string.
    pub fn get_kind_as_string(&self) -> String {
        match self {
            TransactionKind::Income(_) => "Income".to_owned(),
            TransactionKind::Expense(_) => "Expense".to_owned(),
        }
    }

    /// Returns the category of the transaction as a string no matter its type.
    pub fn get_category_as_string(&self) -> String {
        match self {
            TransactionKind::Income(category) => format!("{:?}", category).to_owned(),
            TransactionKind::Expense(category) => format!("{:?}", category).to_owned(),
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
