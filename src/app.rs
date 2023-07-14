use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Arg<'a> {
    Search { input: &'a str },
}

#[function_component(App)]
pub fn app() -> Html {
    let search_input_ref = use_node_ref();
    let search_input = use_state(|| String::new());
    let search_message = use_state(|| String::new());

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
                        to_value(&Arg::Search { input: &*search_input }).unwrap(),
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
        <main class="topnav-and-main">
            // <!-- Top navigation -->
            <section id="top-nav">
                <div id="top-nav-logo">
                    <h1>{"RustyBudget"}</h1>
                </div>
                <div id="top-nav-content">
                    <form class="row" onsubmit={search_callback}>
                        <input type="text" id="search-input" ref={search_input_ref} placeholder="Search..." />
                        <input type="submit" hidden=true />
                    </form>
                </div>
            </section>

            // <!-- Main content -->
            <section id="main">
                <div id="main-sidebar">
                    <div>
                        <a><b>{"Dashboard"}</b></a>
                        <a><b>{"Account Transactions"}</b></a>
                        <a><b>{"Add Cash"}</b></a>
                    </div>
                </div>
                <div id="main-content">
                    <p>{"Recent Transactions"}</p>
                    <p>{"Add Transaction"}</p>
                    <button type="button" class="logo">{"Add Transaction"}</button>
                    <p class="logo"><b>{ &*search_message }</b></p>
                    <img src="public/tauri.svg" class="logo" alt="Test logo"/>
                </div>
            </section>
        </main>
    }
}
