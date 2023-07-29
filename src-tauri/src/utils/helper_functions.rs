//! Helper functions for the application.
use chrono::NaiveDate;
use serde::de::DeserializeOwned;
use std::fs::{read, write};
use std::time::SystemTime;

/// Generates a unique id based on the current time in nanoseconds.
/// # Panics
/// Panics if the current time cannot be retrieved. This should never happen.
pub fn generate_id() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

pub fn format_dollar_to_cents(amount: String) -> String {
    let mut amount = amount;

    // Check if the amount has cents and format it properly
    if amount.contains(".") {
        // Split the amount into dollars and cents and store them in a vector as strings
        let amount_split: Vec<String> = amount.split(".").map(String::from).collect();

        // Make sure the cents are two digits
        match amount_split[1].len() {
            0 => amount.push_str("00"),
            1 => amount.push_str("0"),
            _ => {}
        }

        // Remove the decimal point
        amount = amount.replace(".", "")
    } else {
        // Add two zeros to the end of the amount
        amount.push_str("00");
    }

    // Return the amount in cents
    amount
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
    bincode::deserialize(encoded_obj).expect("Unable to deserialize object")
}

/// Writes a serializable object to a file.
/// # Panics
/// Panics if the object cannot be serialized into a vector of bytes.
/// This happens if the object does not implement the Serialize trait.
///
/// Panics if the file cannot be written to.
pub fn write_file_serialized<T>(path: String, content: T)
where
    T: serde::Serialize,
{
    let serialized_content = serialize(&content);
    // Write the serialized profile to the profile
    write(path.clone(), serialized_content)
        .expect(format!("Unable to write profile to path: {}", path).as_str());
}
