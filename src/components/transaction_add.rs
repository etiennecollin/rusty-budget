use crate::utils::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_updated: Callback<()>,
}

#[function_component(TransactionAdd)]
pub fn transaction_add(props: &Props) -> Html {
    let input_category = NodeRef::default();
    let input_description = NodeRef::default();
    let input_date = NodeRef::default();
    let input_amount = NodeRef::default();
    let input_currency_symbol: NodeRef = NodeRef::default();

    let categories_income_state = use_state_eq(|| (String::new(), Vec::<String>::new()));
    let categories_expense_state = use_state_eq(|| (String::new(), Vec::<String>::new()));
    let currency_symbol_state = use_state_eq(|| String::new());

    // Get values from the backend
    {
        // Clone the states to be used in the async block
        let currency_symbol_state = currency_symbol_state.clone();
        let categories_income_state = categories_income_state.clone();
        let categories_expense_state = categories_expense_state.clone();
        spawn_local(async move {
            // Get the currency symbols and categories from the backend
            currency_symbol_state.set(get_current_account_currency_symbol().await);
            categories_income_state.set(get_transaction_categories("Income".to_owned()).await);
            categories_expense_state.set(get_transaction_categories("Expense".to_owned()).await);
        });
    }

    let on_submit = {
        let input_category = input_category.clone();
        let input_description = input_description.clone();
        let input_date = input_date.clone();
        let input_amount = input_amount.clone();
        let input_currency_symbol = input_currency_symbol.clone();
        let is_updated = props.is_updated.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Get form's HTML elements
            let category_input_element = input_category.cast::<HtmlInputElement>().unwrap();
            let description_input_element = input_description.cast::<HtmlInputElement>().unwrap();
            let date_input_element = input_date.cast::<HtmlInputElement>().unwrap();
            let amount_input_element = input_amount.cast::<HtmlInputElement>().unwrap();
            let currency_symbol_input_element =
                input_currency_symbol.cast::<HtmlInputElement>().unwrap();

            // Extract value from HTML elements
            let kind_category = category_input_element.value();
            let description = description_input_element.value();
            let date = date_input_element.value();
            let mut amount = amount_input_element.value();
            let currency_symbol = currency_symbol_input_element.value();

            // Check if any of the values are empty
            if kind_category.is_empty()
                || description.is_empty()
                || date.is_empty()
                || amount.is_empty()
                || currency_symbol.is_empty()
            {
                return;
            }

            // Clear the form fields
            category_input_element.set_value("");
            description_input_element.set_value("");
            date_input_element.set_value("");
            amount_input_element.set_value("");
            // currency_symbol_input_element.set_value("");

            // Extract the kind and category from the category string
            let kind_category: Vec<_> = kind_category.split("-").collect();
            let kind = kind_category[0].to_owned();
            let category = kind_category[1].to_owned();

            if kind == "Income" {
                // Make the amount positive
                amount = "-".to_owned() + &amount;
            }

            // Add the transaction by calling the backend
            spawn_local(async move {
                invoke(
                    "add_transaction",
                    to_value(&Arg::AddTransaction {
                        kind,
                        category,
                        description,
                        date,
                        amount,
                        currency_symbol,
                    })
                    .unwrap(),
                )
                .await;
            });

            // Tell the parent that the form was submitted and content should be reloaded
            is_updated.emit(());
        })
    };

    // Format the categories as HTML options
    let display_income_categories: VNode = (*categories_income_state)
        .1
        .iter()
        .map(|category| {
            html! {<option value={format!("{}-{}", (*categories_income_state).0, category.clone())}>{category}</option>}
        })
        .collect();
    let display_expense_categories: VNode = (*categories_expense_state)
        .1
        .iter()
        .map(|category| {
            html! {<option value={format!("{}-{}", (*categories_expense_state).0, category.clone())}>{category}</option>}
        })
        .collect();

    // Format the currency symbol as HTML options
    let currency_symbol = (*currency_symbol_state).clone();
    let display_currency_symbol =
        html! {<option value={currency_symbol.clone()} selected=true>{currency_symbol}</option>};

    html! {
        <div class="container">
            <h1>{"Add Transaction"}</h1>
            <form class="col-centered" onsubmit={on_submit}>
                <select ref={input_category} required=true>
                    <option value="" disabled=true selected=true>{"Category"}</option>
                    <optgroup label="Income">
                        {display_income_categories}
                    </optgroup>
                    <optgroup label="Expense">
                        {display_expense_categories}
                    </optgroup>
                </select>
                <input ref={input_description} type="text" placeholder="Description" required=true/>
                <input ref={input_date} type="date" required=true/>
                <input ref={input_amount} type="number" placeholder="Amount" step=".01" min="0" required=true/>
                <select ref={input_currency_symbol} required=true>
                    <option value="" disabled=true selected=false>{"Currency"}</option>
                    {display_currency_symbol}
                </select>
                <button type="submit">{"Add"}</button>
            </form>
        </div>
    }
}
