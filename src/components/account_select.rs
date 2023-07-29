use crate::utils::{invoke, Arg};
use serde_wasm_bindgen::to_value;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_updated: Callback<()>,
}

#[function_component(AccountSelect)]
pub fn account_select(props: &Props) -> Html {
    let input_account_id = NodeRef::default();
    let current_account_id_state = use_state_eq(|| String::new());
    let accounts_state = use_state_eq(|| Vec::<Vec<String>>::new());

    // Get values from the backend
    {
        // Clone the states to be used in the async block
        let current_account_id_state = current_account_id_state.clone();
        let account_state = accounts_state.clone();
        spawn_local(async move {
            // Get the accounts and the current account id from the backend
            account_state.set(
                invoke("get_accounts", to_value(&Arg::Nothing).unwrap())
                    .await
                    .as_string()
                    .unwrap()
                    .split(";")
                    .map(|account| {
                        let account_fields = account.split(",").collect::<Vec<_>>();
                        vec![account_fields[0].to_owned(), account_fields[1].to_owned()]
                    })
                    .collect(),
            );
            current_account_id_state.set(
                invoke("get_current_account_id", to_value(&Arg::Nothing).unwrap())
                    .await
                    .as_string()
                    .unwrap(),
            );
        });
    }

    let on_change = {
        let input_account_id = input_account_id.clone();
        let is_updated = props.is_updated.clone();
        Callback::from(move |_| {
            // e.prevent_default();
            // Get form's HTML elements
            let account_id_input_element = input_account_id.cast::<HtmlInputElement>().unwrap();
            // Extract value from HTML elements
            let account_id = account_id_input_element.value();
            // Check if any of the values are empty
            if account_id.is_empty() {
                return;
            }

            spawn_local(async move {
                invoke(
                    "set_current_account_id",
                    to_value(&Arg::SetCurrentAccountId { account_id }).unwrap(),
                )
                .await;
            });

            // Tell the parent that the form was submitted and content should be reloaded
            is_updated.emit(());
        })
    };

    let display_accounts: VNode = (*accounts_state)
        .iter()
        .map(|account| {
            let is_selected = *current_account_id_state == account[1];
            html! {<option value={account[1].clone()} selected={is_selected}>{account[0].clone()}</option>}
        })
        .collect();

    html! {
        <div class="container">
            <h1>{"Select Account"}</h1>
            <select ref={input_account_id} onchange={on_change} required=true>
                {display_accounts}
            </select>
        </div>
    }
}
