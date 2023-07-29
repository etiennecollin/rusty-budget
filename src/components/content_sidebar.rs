use crate::utils::Page;
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub page_clicked: Callback<Page>,
}

#[function_component(ContentSidebar)]
pub fn content_sidebar(props: &Props) -> Html {
    // Get all of the available pages as links
    let on_click = {
        // Clone the variable to be used in the callback
        let page_clicked = props.page_clicked.clone();
        Callback::from(move |page: String| {
            // Emit the page clicked as a Page enum object
            page_clicked.emit(Page::from_str(&*page).expect("Invalid page"));
        })
    };

    // Generate the links for each page
    let pages: VNode = Page::iter()
        .map(|page| {
            html! {<a onclick={
            let on_click = on_click.clone();
            move |_|{on_click.emit(page.to_string())}}><b>{page.to_string()}</b></a>}
        })
        .collect();

    html! {
        <div id="content-sidebar" class="col">
            {pages}
        </div>
    }
}
