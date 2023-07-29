use crate::components::AccountAdd;
use crate::utils::{invoke, Arg};
use serde_wasm_bindgen::to_value;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_account_available: Callback<bool>,
}

#[function_component(AccountNotAvailable)]
pub fn account_not_available(props: &Props) -> Html {
    let is_account_available = props.is_account_available.clone();

    let on_accounts_updated = {
        let is_account_available = is_account_available.clone();
        Callback::from(move |_| {
            let is_account_available = is_account_available.clone();
            spawn_local(async move {
                // Check if the account is loaded and emit the answer
                is_account_available.emit(
                    invoke("is_account_available", to_value(&Arg::Nothing).unwrap())
                        .await
                        .as_bool()
                        .unwrap(),
                );
            });
        })
    };

    html! {
        <main class="col-centered">
            <h1>{"This profile has no account. Please create one..."}</h1>
            <AccountAdd is_updated={on_accounts_updated}/>
        </main>
    }
}
