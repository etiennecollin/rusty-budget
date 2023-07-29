use crate::{structs::*, utils::*};
use std::fs::{read, write};
use std::str::FromStr;
use strum::IntoEnumIterator;
use tauri::State;
use tauri_api::dialog::{self, Response};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
pub fn search(input: &str) -> String {
    format!("You searched {}.", input)
}

/// Returns a semi-colon-separated list of the current profile accounts as a string.
/// The fields of each account are separated by commas.
/// The fields are: description, balance, currency symbol, and id.
/// # Panics
/// Panics if the current profile cannot be locked.
#[tauri::command(rename_all = "snake_case")]
pub fn get_accounts(app_state: State<AppState>) -> String {
    // Initialize the string used to store the recent transactions
    let mut accounts_string = String::new();

    // Get the transactions from the current account
    let accounts = app_state
        .current_profile
        .lock()
        .expect("Unable to lock profile")
        .get_accounts();

    // Format each transaction in the current account and add it to the string
    for account in accounts {
        accounts_string.push_str(&format!(
            "{},{},{},{:?};",
            account.get_description(),
            account.get_id(),
            account.get_balance().get_amount_display(),
            account.get_balance().get_currency_symbol()
        ));
    }

    // Remove the last semi-colon and return the string
    accounts_string = accounts_string.trim_end_matches(';').to_owned();
    accounts_string
}

/// Returns a semi-colon-separated list of the current account transactions as a string.
/// The fields of each transaction are separated by commas.
/// The fields are: description, transaction type, date, and amount.
/// # Panics
/// Panics if the current account cannot be locked.
#[tauri::command(rename_all = "snake_case")]
pub fn get_transactions(app_state: State<AppState>) -> String {
    // Initialize the string used to store the recent transactions
    let mut recent_transactions = String::new();

    // Get the transactions from the current account
    let current_account_index = *app_state
        .current_account_index
        .lock()
        .expect("Unable to lock account id");
    let transactions = app_state
        .current_profile
        .lock()
        .expect("Unable to lock profile")
        .get_accounts()[current_account_index]
        .get_transactions();

    // Format each transaction in the current account and add it to the string
    for transaction in transactions.iter().rev() {
        recent_transactions.push_str(&format!(
            "{},{},{},{},{};",
            transaction.get_description(),
            transaction.get_kind().get_kind_as_string(),
            transaction.get_kind().get_category_as_string(),
            transaction.get_date(),
            transaction.get_amount().get_amount_display()
        ));
    }

    // Remove the last semi-colon and return the string
    recent_transactions = recent_transactions.trim_end_matches(';').to_owned();
    recent_transactions
}

/// Returns comma-separated expense categories as a string.
#[tauri::command(rename_all = "snake_case")]
pub fn get_transaction_categories(kind: String) -> String {
    // Initialize the string used to store the categoriess
    let mut categories = String::new();

    // Add each category to the string based on the transaction type
    if kind == "Income" {
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

/// Returns the currency symbol of the current account.
/// # Panics
/// Panics if the current account cannot be locked.
#[tauri::command(rename_all = "snake_case")]
pub fn get_current_account_currency_symbol(app_state: State<AppState>) -> String {
    let current_account_index = *app_state
        .current_account_index
        .lock()
        .expect("Unable to lock account id");
    let currency_symbol = app_state
        .current_profile
        .lock()
        .expect("Unable to lock profile")
        .get_accounts()[current_account_index]
        .get_balance()
        .get_currency_symbol();
    currency_symbol.to_string()
}

/// Returns comma-separated currency symbols as a string.
#[tauri::command(rename_all = "snake_case")]
pub fn get_currency_symbols() -> String {
    let mut currency_symbols = String::new();

    for currency_symbol in CurrencySymbol::iter() {
        currency_symbols.push_str(&format!("{:?},", currency_symbol));
    }

    currency_symbols = currency_symbols.trim_end_matches(',').to_owned();
    currency_symbols
}

/// Opens a profile and sets the current profile and account based on the profile serialized in the profile.
/// # Panics
/// Panics if the profile cannot be read, if the current profile or account cannot be locked,
/// if the loaded status of the current profile or account cannot be locked, or if the profile cannot be deserialized.
#[tauri::command(rename_all = "snake_case")]
pub fn open_profile(app_state: State<AppState>) {
    // Get the path of the profile
    let path: String;

    loop {
        // TODO: Move to tauri::api::dialog because tauri_api::dialog is deprecated
        let response = dialog::select(Some("RustyBudget"), Some(".")).unwrap();
        match response {
            Response::Okay(selected_path) => {
                // Set the profile path
                path = selected_path;
                break;
            }
            _ => {}
        }
    }

    // Read the profile and deserialize the profile
    let serialized_profile = read(path.clone()).expect("Unable to read profile");
    let profile: Profile = deserialize(&serialized_profile);

    // Lock the current profile
    let mut current_profile = app_state
        .current_profile
        .lock()
        .expect("Unable to lock profile");
    let mut current_profile_path = app_state
        .current_profile_path
        .lock()
        .expect("Unable to lock profile");
    let mut is_profile_loaded = app_state
        .is_profile_loaded
        .lock()
        .expect("Unable to lock profile status");

    // Set the current profile and set the profile status to loaded
    *current_profile = profile;
    *current_profile_path = path;
    *is_profile_loaded = true;

    // If profile contains accounts, set the current account to the first account
    if current_profile.get_accounts().len() > 0 {
        let mut current_account_index = app_state
            .current_account_index
            .lock()
            .expect("Unable to lock account index");

        let mut is_account_available = app_state
            .is_account_available
            .lock()
            .expect("Unable to lock account status");

        // Set the current account and set the account status to loaded
        *current_account_index = 0;
        *is_account_available = true;
    }
}

/// Creates a new profile with the given name and sets the current profile to the new profile.
/// # Panics
/// Panics if the current profile and its loaded status cannot be locked.
#[tauri::command(rename_all = "snake_case")]
pub fn new_profile(name: String, app_state: State<AppState>) {
    // Initialize the string holding the path
    let path: String;
    loop {
        // TODO: Move to tauri::api::dialog because tauri_api::dialog is deprecated
        let response = dialog::save_file(Some("RustyBudget"), Some(".")).unwrap();
        match response {
            Response::Okay(selected_path) => {
                path = selected_path;
                break;
            }
            _ => {}
        }
    }

    // Lock the current profile
    let mut current_profile = app_state
        .current_profile
        .lock()
        .expect("Unable to lock profile");
    let mut is_profile_loaded = app_state
        .is_profile_loaded
        .lock()
        .expect("Unable to lock profile status");
    // Lock the profile path
    let mut profile_path = app_state
        .current_profile_path
        .lock()
        .expect("Unable to lock profile path");

    // Create a new profile
    *current_profile = Profile::new(name);
    *is_profile_loaded = true;
    *profile_path = path.clone();

    // Write the profile to the profile path
    write_file_serialized(path, (*current_profile).clone())
}
/// Creates a new account and adds it to the current profile.
/// # Panics
/// Panics if the current profile and its loaded status cannot be locked.
#[tauri::command(rename_all = "snake_case")]
pub fn add_account(description: String, currency_symbol: String, app_state: State<AppState>) {
    let account = Account::new(
        description,
        CurrencySymbol::from_str(&*currency_symbol).unwrap(),
    );

    // Lock the current profile and account
    let mut current_profile = app_state
        .current_profile
        .lock()
        .expect("Unable to lock profile");
    let mut current_account_index = app_state
        .current_account_index
        .lock()
        .expect("Unable to lock account index");
    let mut is_account_available = app_state
        .is_account_available
        .lock()
        .expect("Unable to lock account status");
    let current_profile_path = app_state
        .current_profile_path
        .lock()
        .expect("Unable to lock profile path")
        .clone();

    // Add the account to the profile and set the current account
    let new_account_index = (*current_profile).get_accounts_mut().len();
    (*current_profile).add_account(account);
    *current_account_index = new_account_index;
    *is_account_available = true;

    // Write the profile to the profile path
    write_file_serialized(current_profile_path, (*current_profile).clone())
}

/// Adds a transaction to the current account.
/// # Panics
/// Panics if the current account cannot be locked, if the transaction type is invalid,
/// if the category is invalid, if the date is invalid, or if the amount is invalid.
#[tauri::command(rename_all = "snake_case")]
pub fn add_transaction(
    kind: String,
    category: String,
    description: String,
    date: String,
    amount: String,
    currency_symbol: String,
    app_state: State<AppState>,
) {
    // Parse the transaction type
    let kind_parsed: TransactionKind;
    if kind == "Income" {
        kind_parsed = TransactionKind::Income(
            IncomeCategory::from_str(&*category).expect("Invalid income category"),
        );
    } else {
        kind_parsed = TransactionKind::Expense(
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

    // Lock the current profile and account

    let mut current_profile = app_state
        .current_profile
        .lock()
        .expect("Unable to lock profile");
    let current_account_index = *app_state
        .current_account_index
        .lock()
        .expect("Unable to lock account index");
    let current_account = &mut current_profile.get_accounts_mut()[current_account_index];
    let current_profile_path = app_state
        .current_profile_path
        .lock()
        .expect("Unable to lock profile path")
        .clone();

    // Add the transaction to the current account
    current_account.add_transaction(Transaction::new(
        kind_parsed.clone(),
        description.clone(),
        date.clone(),
        amount_parsed.clone(),
    ));

    // Write the profile to the profile path
    write_file_serialized(current_profile_path, (*current_profile).clone())
}

/// Gets the current account id and returns it as a string.
#[tauri::command(rename_all = "snake_case")]
pub fn get_current_account_id(app_state: State<AppState>) -> String {
    // Lock the current account
    let current_account_index = app_state
        .current_account_index
        .lock()
        .expect("Unable to lock account index");

    current_account_index.to_string()
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_current_account_id(account_id: String, app_state: State<AppState>) {
    // Lock the current profile and account
    let mut current_account_index = app_state
        .current_account_index
        .lock()
        .expect("Unable to lock account index");

    let index = account_id.parse::<usize>().expect("Invalid account index");

    // Set the current account index
    *current_account_index = index;
}

/// Returns whether or not a profile is loaded.
/// # Panics
/// Panics if the is_profile_loaded variable cannot be locked.
#[tauri::command(rename_all = "snake_case")]
pub fn is_profile_loaded(app_state: State<AppState>) -> bool {
    app_state
        .is_profile_loaded
        .lock()
        .expect("Cannot lock is_profile_loaded")
        .clone()
}

/// Returns whether or not an account is available and loaded.
/// # Panics
/// Panics if the is_account_available variable cannot be locked.
#[tauri::command(rename_all = "snake_case")]
pub fn is_account_available(app_state: State<AppState>) -> bool {
    app_state
        .is_account_available
        .lock()
        .expect("Cannot lock is_account_available")
        .clone()
}
