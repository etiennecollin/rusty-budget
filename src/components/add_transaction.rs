use crate::utils::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_updated: Callback<()>,
}

#[function_component(AddTransaction)]
pub fn add_transaction(props: &Props) -> Html {
    let input_category = NodeRef::default();
    let input_description = NodeRef::default();
    let input_date = NodeRef::default();
    let input_amount = NodeRef::default();
    let input_currency_symbol: NodeRef = NodeRef::default();
    let display_categories = use_get_transaction_categories("Expense".to_owned());
    let display_currency_symbols = use_get_currency_symbols();

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
            let category = category_input_element.value();
            let description = description_input_element.value();
            let date = date_input_element.value();
            let amount = amount_input_element.value();
            let currency_symbol = currency_symbol_input_element.value();

            // Check if any of the values are empty
            if category.is_empty()
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
            currency_symbol_input_element.set_value("");

            // Add the transaction by calling the backend
            spawn_local(async move {
                invoke(
                    "add_transaction",
                    to_value(&Arg::AddTransaction {
                        transaction_type: "Expense".to_owned(),
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

    html! {
        <div id="content-main-right" class="container">
            <h1>{"Add Transaction"}</h1>
            <form id="add-transaction" onsubmit={on_submit}>
                    <select ref={input_category} id="transaction-category" required=true>
                        <option value="" disabled=true selected=true>{"Category"}</option>
                        {display_categories}
                    </select>
                <input ref={input_description} type="text" placeholder="Description" required=true/>
                <input ref={input_date} type="date" required=true/>
                <input ref={input_amount} type="number" placeholder="Amount" step=".01" required=true/>
                    <select ref={input_currency_symbol} id="currency-symbol" required=true>
                        <option value="" disabled=true selected=true>{"Currency"}</option>
                        {display_currency_symbols}
                    </select>
                <button type="submit">{"Add Transaction"}</button>
            </form>
        </div>
    }
}
