use crate::components::TopBarContent;
use yew::prelude::*;

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    html! {
        <section id="top-bar">
            <div id="top-bar-logo" class="row-centered">
                <img src="public/logo.png" class="logo" alt="RustyBudget logo"/>
                <h1>{"RustyBudget"}</h1>
            </div>
            <TopBarContent/>
        </section>
    }
}
