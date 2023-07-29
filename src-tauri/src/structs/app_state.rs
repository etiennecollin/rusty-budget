use super::Profile;
use std::sync::Mutex;

/// AppState is a struct that holds the state of the application
pub struct AppState {
    pub is_profile_loaded: Mutex<bool>,
    pub current_profile: Mutex<Profile>,
    pub is_account_available: Mutex<bool>,
    pub current_account_index: Mutex<usize>,
    pub current_profile_path: Mutex<String>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            is_profile_loaded: Mutex::new(false),
            current_profile: Mutex::new(Profile::default()),
            is_account_available: Mutex::new(false),
            current_account_index: Mutex::new(0),
            current_profile_path: Mutex::new(String::new()),
        }
    }
}
