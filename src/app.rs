use rusty_budget_ui::{
    components::{ProfileLoaded, ProfileNotLoaded},
    utils::{invoke, Arg},
};
use serde_wasm_bindgen::to_value;
use yew::{platform::spawn_local, prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    let is_profile_loaded = use_state_eq(|| false);

    {
        let is_profile_loaded = is_profile_loaded.clone();
        spawn_local(async move {
            // Check if the account is available and emit the answer
            is_profile_loaded.set(
                invoke("is_profile_loaded", to_value(&Arg::Nothing).unwrap())
                    .await
                    .as_bool()
                    .unwrap(),
            );
        });
    }

    let on_is_profile_loaded = {
        let is_profile_loaded = is_profile_loaded.clone();
        Callback::from(move |is_loaded: bool| {
            is_profile_loaded.set(is_loaded);
        })
    };

    if *is_profile_loaded {
        html! {
            <ProfileLoaded/>
        }
    } else {
        html! {
            <ProfileNotLoaded is_profile_loaded={on_is_profile_loaded}/>
        }
    }
}
