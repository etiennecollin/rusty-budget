use crate::{structs::*, utils::*};
use std::fs::{read, write};
use std::str::FromStr;
use strum::IntoEnumIterator;
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn search(input: &str) -> String {
    format!("You searched {}.", input)
}

/// Returns comma-separated expense categories as a string.
#[tauri::command(rename_all = "snake_case")]
pub fn get_transaction_categories(transaction_type: String) -> String {
    // Initialize the string used to store the categoriess
    let mut categories = String::new();

    // Add each category to the string based on the transaction type
    if transaction_type == "Income" {
        for category in IncomeCategory::iter() {
            categories.push_str(&format!("{:?},", category));
        }
    } else {
        for category in ExpenseCategory::iter() {
            categories.push_str(&format!("{:?},", category));
        }
    }

    // Remove the last comma and return the string
    categories = categories.trim_end_matches(',').to_owned();
    categories
}

/// Returns comma-separated currency symbols as a string.
/// # Panics
/// Panics if the current account cannot be locked.
#[tauri::command]
pub fn get_currency_symbols(app_state: State<AppState>) -> String {
    let currency_symbol = app_state
        .current_account
        .lock()
        .expect("Unable to lock account")
        .clone()
        .get_balance()
        .get_currency_symbol();
    currency_symbol.to_string()

    // let mut currency_symbols = String::new();

    // for currency_symbol in CurrencySymbol::iter() {
    //     currency_symbols.push_str(&format!("{:?},", currency_symbol));
    // }

    // currency_symbols = currency_symbols.trim_end_matches(',').to_owned();
    // currency_symbols
}

/// Adds a transaction to the current account.
/// # Panics
/// Panics if the current account cannot be locked, if the transaction type is invalid,
/// if the category is invalid, if the date is invalid, or if the amount is invalid.
#[tauri::command(rename_all = "snake_case")]
pub fn add_transaction(
    transaction_type: String,
    category: String,
    description: String,
    date: String,
    amount: String,
    currency_symbol: String,
    app_state: State<AppState>,
) {
    // Parse the transaction type
    let transaction_type_parsed: TransactionType;
    if transaction_type == "Income" {
        transaction_type_parsed = TransactionType::Income(
            IncomeCategory::from_str(&*category).expect("Invalid income category"),
        );
    } else {
        transaction_type_parsed = TransactionType::Expense(
            ExpenseCategory::from_str(&*category).expect("Invalid expense category"),
        );
    }

    // Parse the currency symbol
    let currency_symbol_parsed =
        CurrencySymbol::from_str(&*currency_symbol).expect("Invalid currency symbol");
    // Convert the amount to cents
    let amount_cents = format_dollar_to_cents(amount)
        .parse::<i64>()
        .expect("Invalid amount");
    // Convert the amount to an Amount struct with the correct currency symbol
    let amount_parsed = Amount::new(amount_cents, currency_symbol_parsed);

    // Add the transaction to the current account
    app_state
        .current_account
        .lock()
        .expect("Unable to lock account")
        .add_transaction(Transaction::new(
            transaction_type_parsed.clone(),
            description.clone(),
            date.clone(),
            amount_parsed.clone(),
        ));
}

/// Returns a semi-colon-separated list of the current account transactions as a string.
/// The fields of each transaction are separated by commas.
/// The fields are: description, transaction type, date, and amount.
/// # Panics
/// Panics if the current account cannot be locked.
#[tauri::command]
pub fn get_transactions(app_state: State<AppState>) -> String {
    // Initialize the string used to store the recent transactions
    let mut recent_transactions = String::new();

    // Get the transactions from the current account
    let transactions = app_state
        .current_account
        .lock()
        .expect("Unable to lock account")
        .clone()
        .get_transactions();

    // Format each transaction in the current account and add it to the string
    for transaction in transactions {
        recent_transactions.push_str(&format!(
            "{},{:?},{},{};",
            transaction.get_description(),
            transaction.get_transaction_type(),
            transaction.get_date(),
            transaction.get_amount().get_amount_display()
        ));
    }

    // Remove the last semi-colon and return the string
    recent_transactions = recent_transactions.trim_end_matches(';').to_owned();
    recent_transactions
}

/// Returns a semi-colon-separated list of the current file accounts as a string.
/// The fields of each account are separated by commas.
/// The fields are: description, balance, currency symbol, and id.
/// # Panics
/// Panics if the current file cannot be locked.
#[tauri::command]
pub fn get_accounts(app_state: State<AppState>) -> String {
    // Initialize the string used to store the recent transactions
    let mut accounts_string = String::new();

    // Get the transactions from the current account
    let accounts = app_state
        .current_file
        .lock()
        .expect("Unable to lock file")
        .clone()
        .get_accounts();

    // Format each transaction in the current account and add it to the string
    for account in accounts {
        accounts_string.push_str(&format!(
            "{},{},{},{:?};",
            account.get_description(),
            account.get_balance().get_amount_display(),
            account.get_balance().get_currency_symbol(),
            account.get_id()
        ));
    }

    // Remove the last semi-colon and return the string
    accounts_string = accounts_string.trim_end_matches(';').to_owned();
    accounts_string
}

/// Saves the current file to a file.
/// # Panics
/// Panics if the file cannot be written to, if the current file cannot be locked,
/// or if the file cannot be serialized.
#[tauri::command]
pub fn save_file(path: String, app_state: State<AppState>) {
    // Get the current file and serialize it
    let current_file = app_state.current_file.lock().expect("Unable to lock file");
    let serialized_file = serialize(&*current_file);

    // Write the serialized file to the file
    write(path, serialized_file).expect("Unable to write file");
}

/// Opens a file and sets the current file and account based on the file serialized in the file.
/// # Panics
/// Panics if the file cannot be read, if the current file or account cannot be locked,
/// if the loaded status of the current file or account cannot be locked, or if the file cannot be deserialized.
#[tauri::command]
pub fn open_file(path: String, app_state: State<AppState>) {
    // Read the file and deserialize the file
    let serialized_file = read(path).expect("Unable to read file");
    let file: File = deserialize(&serialized_file);

    // Lock the current file
    let mut current_file = app_state.current_file.lock().expect("Unable to lock file");
    let mut is_file_loaded = app_state
        .is_file_loaded
        .lock()
        .expect("Unable to lock file status");

    // Set the current file and set the file status to loaded
    *current_file = file;
    *is_file_loaded = true;

    // If file contains accounts, set the current account to the first account
    if current_file.get_accounts().len() > 0 {
        // Lock the current account
        let mut current_account = app_state
            .current_account
            .lock()
            .expect("Unable to lock account");
        let mut is_account_loaded = app_state
            .is_account_loaded
            .lock()
            .expect("Unable to lock account status");

        // Set the current account and set the account status to loaded
        *current_account = current_file.get_accounts()[0].clone();
        *is_account_loaded = true;
    }
}

/// Creates a new file with the given name and sets the current file to the new file.
/// # Panics
/// Panics if the current file and its loaded status cannot be locked.
#[tauri::command]
pub fn new_file(name: String, app_state: State<AppState>) {
    // Create a new file
    let file = File::new(name);

    // Lock the current file
    let mut current_file = app_state.current_file.lock().expect("Unable to lock file");
    let mut is_file_loaded = app_state
        .is_file_loaded
        .lock()
        .expect("Unable to lock file status");

    // Set the current file
    *current_file = file;
    *is_file_loaded = true;
}

/// Returns whether or not a file is loaded.
/// # Panics
/// Panics if the is_file_loaded variable cannot be locked.
#[tauri::command]
pub fn is_file_loaded(app_state: State<AppState>) -> bool {
    app_state
        .is_file_loaded
        .lock()
        .expect("Cannot lock is_file_loaded")
        .clone()
}

/// Returns whether or not an account is selected.
/// # Panics
/// Panics if the is_account_loaded variable cannot be locked.
#[tauri::command]
pub fn is_account_loaded(app_state: State<AppState>) -> bool {
    app_state
        .is_account_loaded
        .lock()
        .expect("Cannot lock is_account_loaded")
        .clone()
}
