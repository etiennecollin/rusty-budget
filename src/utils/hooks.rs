use super::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::hook;
use yew::{prelude::*, virtual_dom::VNode};

/// Returns a VNode containing HTML options for each expense category.
#[hook]
pub fn use_get_transaction_categories(transaction_type: String) -> VNode {
    // Store the categories as a vector of strings
    let categories_state = use_state_eq(|| Vec::<String>::new());

    // Fetch the categories from the backend
    {
        let categories_state = categories_state.clone();
        spawn_local(async move {
            let categories: Vec<String> = invoke(
                "get_transaction_categories",
                to_value(&Arg::GetTransactionCategories { transaction_type }).unwrap(),
            )
            .await
            .as_string()
            .unwrap()
            .split(",")
            .map(String::from)
            .collect();
            categories_state.set(categories);
        });
    }

    // Format the categories as HTML options
    let display_categories = categories_state
        .iter()
        .map(
            |category| html! {<option value={(&*category).clone()}>{(&*category).clone()}</option>},
        )
        .collect();

    // Return the list of HTML options
    display_categories
}

/// Returns a VNode containing HTML options for each currency symbol.
#[hook]
pub fn use_get_currency_symbols() -> VNode {
    // Store the currency symbols as a vector of strings
    let currency_symbols_state = use_state_eq(|| Vec::<String>::new());

    // Fetch the currency symbols from the backend
    {
        let currency_symbols_state = currency_symbols_state.clone();
        spawn_local(async move {
            let currency_symbols: Vec<String> =
                invoke("get_currency_symbols", to_value(&Arg::Nothing).unwrap())
                    .await
                    .as_string()
                    .unwrap()
                    .split(",")
                    .map(String::from)
                    .collect();
            currency_symbols_state.set(currency_symbols);
        });
    }

    // Format the currency symbols as HTML options
    let display_currency_symbols = currency_symbols_state
        .iter()
        .map(
            |currency| html! {<option value={(&*currency).clone()} selected=true>{(&*currency).clone()}</option>},
        )
        .collect();

    // Return the list of HTML options
    display_currency_symbols
}
