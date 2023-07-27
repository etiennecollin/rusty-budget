use rusty_budget_ui::components::*;
use rusty_budget_ui::utils::*;
use serde_wasm_bindgen::to_value;
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // Verify that the file is loaded
    let is_file_loaded = use_state_eq(|| false);
    {
        let is_file_loaded = is_file_loaded.clone();
        use_effect(move || {
            spawn_local(async move {
                is_file_loaded.set(
                    invoke("is_file_loaded", to_value(&Arg::Nothing).unwrap())
                        .await
                        .as_bool()
                        .unwrap(),
                );
            });
            || {}
        });
    }

    let open_file = {
        Callback::from(move |_| {
            // TODO open dialog to select file
            todo!();
            let path: String = "temp".to_owned();

            spawn_local(async move {
                invoke("open_file", to_value(&Arg::IOFile { path }).unwrap()).await;
            });
        })
    };

    let new_file = {
        Callback::from(move |_| {
            // TODO create new file with custom page
            todo!();
            let path: String = "temp".to_owned();

            spawn_local(async move {
                invoke("new_file", to_value(&Arg::IOFile { path }).unwrap()).await;
            });
        })
    };

    if *is_file_loaded {
        html! {
            <main id="main">
                // <!-- Top navigation -->
                <section id="top-bar">
                    <div id="top-bar-logo" class="row-centered">
                        <img src="public/logo.png" class="logo" alt="RustyBudget logo"/>
                        <h1>{"RustyBudget"}</h1>
                    </div>
                    <TopBarContent/>
                </section>

                // <!-- Main content -->
                <section id="content">
                    <ContentSidebar/>
                    <ContentMain/>
                </section>
            </main>
        }
    } else {
        html! {
            <main id="load-file">
                <h1>{"Please load a file..."}</h1>
                <button onclick={open_file}>
                    {"Open file"}
                </button>
                <button onclick={new_file}>
                    {"New file"}
                </button>
            </main>
        }
    }
}
