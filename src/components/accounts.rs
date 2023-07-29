use yew::prelude::*;

use crate::components::AccountAdd;

#[function_component(Accounts)]
pub fn accounts() -> Html {
    let reload = use_state(|| false);

    let on_accounts_updated = {
        let reload = reload.clone();
        Callback::from(move |_| {
            reload.set(!*reload);
        })
    };
    html! {
        <>
            <h1>{"Accounts"}</h1>
            // <AccountsRecent reload={*reload}/>
            <AccountAdd is_updated={on_accounts_updated}/>
        </>
    }
}
