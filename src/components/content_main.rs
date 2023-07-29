use crate::{
    components::{accounts::Accounts, Budget, Stats, Transactions},
    utils::Page,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub page: Page,
}
#[function_component(ContentMain)]
pub fn content_main(props: &Props) -> Html {
    match props.page {
        Page::Transactions => {
            html! {
                <Transactions/>
            }
        }
        Page::Accounts => {
            html! {
                <Accounts/>
            }
        }
        Page::Budget => {
            html! {
                <Budget/>
            }
        }
        Page::Stats => {
            html! {
                <Stats/>
            }
        }
    }
}
