use super::*;
use serde_wasm_bindgen::to_value;
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
        return html! {<tr><td colspan=4>{"No recent transactions"}</td></tr>};
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
