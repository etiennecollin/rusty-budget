use serde_wasm_bindgen::to_value;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*, virtual_dom::VNode};

use crate::utils::{get_currency_symbols, invoke, Arg};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_updated: Callback<()>,
}

#[function_component(AccountAdd)]
pub fn account_add(props: &Props) -> Html {
    let input_description = NodeRef::default();
    let input_currency_symbol = NodeRef::default();
    let currency_symbols_state = use_state_eq(|| Vec::<String>::new());

    let on_submit = {
        // Clone the variables to be used in the callback
        let input_description = input_description.clone();
        let input_currency_symbol = input_currency_symbol.clone();
        let is_updated = props.is_updated.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Get form's HTML elements
            let description_input_element = input_description.cast::<HtmlInputElement>().unwrap();
            let currency_symbol_input_element =
                input_currency_symbol.cast::<HtmlInputElement>().unwrap();

            // Extract value from HTML elements
            let description = description_input_element.value();
            let currency_symbol = currency_symbol_input_element.value();

            // Check if any of the values are empty
            if description.is_empty() || currency_symbol.is_empty() {
                return;
            }

            // Clear the form fields
            description_input_element.set_value("");
            currency_symbol_input_element.set_value("");

            // Create the add account
            spawn_local(async move {
                invoke(
                    "add_account",
                    to_value(&Arg::AddAccount {
                        description,
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

    // Fetch the currency symbols from the backend
    {
        let currency_symbols_state = currency_symbols_state.clone();
        spawn_local(async move {
            currency_symbols_state.set(get_currency_symbols().await);
        });
    }

    // Format the currency symbols as HTML options
    let display_currency_symbols: VNode = (*currency_symbols_state)
        .iter()
        .map(
            |currency| html! {<option value={(&*currency).clone()}>{(&*currency).clone()}</option>},
        )
        .collect();

    html! {
        <form class="row-centered" onsubmit={on_submit}>
            <div>
                <input ref={input_description} type="text" placeholder="Account Description" required=true/>
                <select ref={input_currency_symbol} required=true>
                    <option value="" disabled=true selected=true>{"Currency"}</option>
                    {display_currency_symbols}
                </select>
            </div>
            <button type="submit">{"New Account"}</button>
        </form>
    }
}
