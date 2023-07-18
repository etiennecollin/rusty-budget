/// TransactionType is an enum that represents the type of transaction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransactionType {
    Income(IncomeCategory),
    Expense(ExpenseCategory),
}

/// Implements the TransactionType struct.
impl TransactionType {
    /// Returns the type of the transaction as a string.
    pub fn get_type_as_string(&self) -> String {
        match self {
            TransactionType::Income(_) => "Income".to_string(),
            TransactionType::Expense(_) => "Expense".to_string(),
        }
    }

    /// Returns the category of the transaction as a string no matter its type.
    pub fn get_category_as_string(&self) -> String {
        match self {
            TransactionType::Income(category) => format!("{:?}", category).to_string(),
            TransactionType::Expense(category) => format!("{:?}", category).to_string(),
        }
    }
}

/// IncomeCategory is an enum that represents the category of income.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IncomeCategory {
    Salary,
    Gift,
    Other,
}

/// ExpenseCategory is an enum that represents the category of expense.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
