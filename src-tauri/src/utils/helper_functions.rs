//! Helper functions for the application.

use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{time::SystemTime, vec};

/// Generates a unique id based on the current time in nanoseconds.
/// # Panics
/// Panics if the current time cannot be retrieved. This should never happen.
pub fn generate_id() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

/// Verifies that a date is properly formatted.
/// The date must be in the format: 'YYYY-MM-DD' and be valid.
/// For example: '2021-11-30'.
/// # Panics
/// Panics if the date is invalid or improperly formatted.
pub fn verify_date_format(date: String) -> String {
    let date: Result<NaiveDate, chrono::ParseError> =
        NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d");

    match date {
        Ok(_) => date.unwrap().to_string(),
        Err(_) => panic!("Invalid date"),
    }
}

/// Serializes a struct into a vector of bytes.
/// # Panics
/// Panics if the struct cannot be serialized into a vector of bytes.
/// This happens if the struct does not implement the Serialize trait.
pub fn serialize(obj: &impl serde::Serialize) -> Vec<u8> {
    bincode::serialize(&obj).unwrap()
}

/// Deserializes a vector of bytes into a struct.
/// # Panics
/// Panics if the vector of bytes cannot be deserialized into the struct.
/// This happens if the struct does not implement the Deserialize trait.
pub fn deserialize<T>(encoded_obj: &Vec<u8>) -> T
where
    T: DeserializeOwned,
{
    bincode::deserialize(encoded_obj).unwrap()
}
