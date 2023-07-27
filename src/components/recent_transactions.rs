use crate::utils::helper_functions::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub reload: bool,
}

#[function_component(RecentTransactions)]
pub fn recent_transactions(props: &Props) -> Html {
    let recent_transactions_state = use_state_eq(|| String::new());
    // Get the recent transactions from the server
    {
        let recent_transactions_state = recent_transactions_state.clone();
        use_effect(move || {
            spawn_local(async move {
                recent_transactions_state.set(get_recent_transactions().await);
            });
            || {}
        });
    }

    // Format the recent transactions as HTML table rows
    let display_recent_transactions = parse_transactions(&recent_transactions_state);

    html! {
        <div id="content-main-left" class="container">
            <h1>{"Recent Transactions"}</h1>
            <div>
                <table id="recent-transactions">
                    <thead>
                        <th id="recent-transactions-description">{"Description"}</th>
                        <th id="recent-transactions-category">{"Category"}</th>
                        <th id="recent-transactions-date">{"Date"}</th>
                        <th id="recent-transactions-amount">{"Amount"}</th>
                    </thead>
                    <tbody id="recent-transactions-body">
                        {display_recent_transactions}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
