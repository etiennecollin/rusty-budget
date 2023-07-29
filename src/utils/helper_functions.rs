use super::*;
use serde_wasm_bindgen::to_value;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub async fn get_recent_transactions() -> String {
    // Get the recent transactions from the backend
    let recent_transactions = invoke("get_transactions", to_value(&Arg::Nothing).unwrap())
        .await
        .as_string()
        .unwrap();

    // Return the list of HTML options
    recent_transactions
}

/// Returns a VNode containing HTML table rows of the recent transactions.
pub fn parse_transactions(transactions: &String) -> VNode {
    // Check if there are recent transactions
    if transactions.is_empty() {
        return html! {<tr><td colspan=5>{"No recent transactions"}</td></tr>};
    }

    // Convert the recent transactions to a vector of vectors of strings
    let transactions_vectorized: Vec<Vec<String>> = transactions
        .split(";") // Separate transactions, then fields
        .map(|transaction| transaction.split(",").map(String::from).collect())
        .collect();

    // Convert the recent transactions to HTML table rows
    let transactions_html = transactions_vectorized
        .iter()
        .map(|transaction| {
            html! {<tr>{transaction.iter().map(
                |transaction_field| { html! {<td>{transaction_field}</td>} }
            )
            .collect::<Vec<_>>()}</tr>}
        })
        .collect();

    // Return the list of HTML table rows
    transactions_html
}

/// Returns a tuple containing the transaction kind as a string
/// and a vector with each transaction category as a strings.
pub async fn get_transaction_categories(kind: String) -> (String, Vec<String>) {
    // Fetch the categories from the backend
    let categories = invoke(
        "get_transaction_categories",
        to_value(&Arg::GetTransactionCategories { kind: kind.clone() }).unwrap(),
    )
    .await
    .as_string()
    .unwrap()
    .split(",")
    .map(String::from)
    .collect();

    // Return the tuple
    (kind, categories)

}

/// Returns a list containing each currency symbol as a strings.
pub async fn get_currency_symbols() -> Vec<String> {
    // Store the currency symbols as a vector of strings
    // Fetch the currency symbols from the backend
    let currency_symbols = invoke("get_currency_symbols", to_value(&Arg::Nothing).unwrap())
        .await
        .as_string()
        .unwrap()
        .split(",")
        .map(String::from)
        .collect();

    // Return the list
    currency_symbols
}

/// Returns a the currency symbol of the current account as a string.
pub async fn get_current_account_currency_symbol() -> String {
    // Store the currency symbols as a vector of strings
    let currency_symbol = invoke(
        "get_current_account_currency_symbol",
        to_value(&Arg::Nothing).unwrap(),
    )
    .await
    .as_string()
    .unwrap();

    // Return the string
    currency_symbol
}
