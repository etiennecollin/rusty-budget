use crate::components::{AddTransaction, ListComponent, RecentTransactions, TestComponent};
use yew::prelude::*;

#[function_component(ContentMain)]
pub fn content_main() -> Html {
    let reload = use_state(|| false);

    let on_transactions_updated = {
        let reload = reload.clone();
        Callback::from(move |_| {
            reload.set(true);
        })
    };

    html! {
        <>
            <RecentTransactions reload={*reload}/>
            <AddTransaction is_updated={on_transactions_updated}/>
        </>
    }
}
