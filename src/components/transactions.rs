use crate::components::{AccountSelect, TransactionAdd, TransactionsRecent};
use yew::prelude::*;

#[function_component(Transactions)]
pub fn transactions() -> Html {
    let reload = use_state(|| false);

    let on_transactions_updated = {
        let reload = reload.clone();
        Callback::from(move |_| {
            reload.set(!*reload);
        })
    };

    html! {
        <>
            <TransactionsRecent reload={*reload}/>
            <div id="content-main-right">
                <AccountSelect is_updated={on_transactions_updated.clone()}/>
                <TransactionAdd is_updated={on_transactions_updated}/>
            </div>
        </>
    }
}
