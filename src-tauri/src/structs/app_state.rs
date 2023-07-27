use super::{Account, File};
use std::sync::Mutex;

/// AppState is a struct that holds the state of the application
pub struct AppState {
    pub is_file_loaded: Mutex<bool>,
    pub current_file: Mutex<File>,
    pub is_account_loaded: Mutex<bool>,
    pub current_account: Mutex<Account>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            is_file_loaded: Mutex::new(false),
            current_file: Mutex::new(File::default()),
            is_account_loaded: Mutex::new(false),
            current_account: Mutex::new(Account::default()),
        }
    }
}