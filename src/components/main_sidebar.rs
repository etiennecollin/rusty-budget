use yew::prelude::*;

#[function_component(ContentSidebar)]
pub fn content_sidebar() -> Html {
    html! {
        <>
            <div id="content-sidebar" class="col">
                <a><b>{"Dashboard"}</b></a>
                <a><b>{"Account Transactions"}</b></a>
                <a><b>{"Add Cash"}</b></a>
            </div>
        </>
    }
}
