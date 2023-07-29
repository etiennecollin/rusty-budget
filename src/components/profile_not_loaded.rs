use serde_wasm_bindgen::to_value;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::utils::{invoke, Arg};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_profile_loaded: Callback<bool>,
}

#[function_component(ProfileNotLoaded)]
pub fn profile_not_loaded(props: &Props) -> Html {
    let is_profile_loaded = props.is_profile_loaded.clone();

    let open_profile = {
        // Clone the variable to be used in the callback
        let is_profile_loaded = is_profile_loaded.clone();
        Callback::from(move |_| {
            // Clone the variable to be used in the async block
            let is_profile_loaded = is_profile_loaded.clone();
            spawn_local(async move {
                // Open the profile
                invoke("open_profile", to_value(&Arg::Nothing).unwrap()).await;

                // Check if the profile is loaded and emit the answer
                is_profile_loaded.emit(
                    invoke("is_profile_loaded", to_value(&Arg::Nothing).unwrap())
                        .await
                        .as_bool()
                        .unwrap(),
                );
            });
        })
    };

    let input_name = NodeRef::default();
    let new_profile = {
        // Clone the variables to be used in the callback
        let is_profile_loaded = is_profile_loaded.clone();
        let input_name = input_name.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Get form's HTML elements
            let name_input_element = input_name.cast::<HtmlInputElement>().unwrap();
            // Extract value from HTML elements
            let name = name_input_element.value();
            // Check if any of the values are empty
            if name.is_empty() {
                return;
            }
            // Clear the form fields
            name_input_element.set_value("");

            // Clone the variable to be used in the async block
            let is_profile_loaded = is_profile_loaded.clone();
            spawn_local(async move {
                // Create the new profile
                invoke("new_profile", to_value(&Arg::NewProfile { name }).unwrap()).await;

                // Check if the profile is loaded and emit the answer
                is_profile_loaded.emit(
                    invoke("is_profile_loaded", to_value(&Arg::Nothing).unwrap())
                        .await
                        .as_bool()
                        .unwrap(),
                );
            });
        })
    };

    html! {
        <main class="col-centered">
            <h1>{"Please load or create a profile..."}</h1>
            <button onclick={open_profile}>
                {"Open profile"}
            </button>
            <form class="row-centered" onsubmit={new_profile}>
                <input ref={input_name} type="text" placeholder="Profile Name" required=true/>
                <button type="submit">{"New Profile"}</button>
            </form>
        </main>
    }
}
