use crate::{
    components::{ContentMain, ContentSidebar, TopBar},
    utils::Page,
};
use yew::prelude::*;

#[function_component(AccountAvailable)]
pub fn account_available() -> Html {
    let page = use_state(|| Page::default());

    let on_page_clicked = {
        let page = page.clone();
        Callback::from(move |page_clicked: Page| {
            page.set(page_clicked);
        })
    };
    html! {
        <>
            <TopBar/>
            <section id="content">
                <ContentSidebar page_clicked={on_page_clicked}/>
                <ContentMain page={*page}/>
            </section>
        </>
    }
}
