use crate::{
    components::{AccountAvailable, AccountNotAvailable},
    utils::{invoke, Arg},
};
use serde_wasm_bindgen::to_value;
use yew::{platform::spawn_local, prelude::*};

#[function_component(ProfileLoaded)]
pub fn profile_loaded() -> Html {
    let is_account_available = use_state_eq(|| false);

    // Check if the account is available in backend
    {
        let is_account_available = is_account_available.clone();
        spawn_local(async move {
            // Check if the account is available and emit the answer
            is_account_available.set(
                invoke("is_account_available", to_value(&Arg::Nothing).unwrap())
                    .await
                    .as_bool()
                    .unwrap(),
            );
        });
    }

    let on_is_account_available = {
        let is_account_available = is_account_available.clone();
        Callback::from(move |is_available: bool| {
            is_account_available.set(is_available);
        })
    };

    if *is_account_available {
        html! {
            <main id="main">
                <AccountAvailable/>
            </main>
        }
    } else {
        html! {
            <AccountNotAvailable is_account_available={on_is_account_available}/>
        }
    }
}
