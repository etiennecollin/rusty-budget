use serde_wasm_bindgen::to_value;
use yew::{platform::spawn_local, prelude::*};

use crate::utils::{invoke, Arg};

#[function_component(TopBarContent)]
pub fn top_bar_content() -> Html {
    let search_input_ref = use_node_ref();
    let search_input = use_state_eq(|| String::new());
    let search_message = use_state_eq(|| String::new());

    {
        let search_input = search_input.clone();
        let search_input_dep = search_input.clone();
        let search_message = search_message.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if search_input.is_empty() {
                        return;
                    }

                    let new_message = invoke(
                        "search",
                        to_value(&Arg::Search {
                            input: search_input.to_string(),
                        })
                        .unwrap(),
                    )
                    .await
                    .as_string()
                    .unwrap();
                    search_message.set(new_message);
                });
                || {}
            },
            search_input_dep,
        );
    }

    let search_callback = {
        let search_input = search_input.clone();
        let search_input_ref = search_input_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            search_input.set(
                search_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <>
            <div id="top-bar-content" class="row-centered">
                <form onsubmit={search_callback}>
                    <input type="text" ref={search_input_ref} placeholder="Search..." />
                    <input type="submit" hidden=true />
                </form>
                <p><b>{ &*search_message }</b></p>
            </div>
        </>
    }
}
