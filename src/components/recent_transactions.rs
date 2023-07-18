use web_sys::{HtmlInputElement, MouseEvent};
use yew::{html, Component, Html, InputEvent, NodeRef, Properties, TargetCast};

// pub enum Msg {
//     SetInput(u32),
//     AddValue(u32),
//     DoNothing,
// }

// #[derive(Properties, PartialEq)]
// pub struct RecentTransactionsProps {
//     pub min_value: u32,
//     pub max_value: u32,
// }

pub struct RecentTransactions {
    input_ref: NodeRef,
}

impl Component for RecentTransactions {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Recent Transactions"}</h1>
                <table id="recent-transactions">
                    <thead>
                        <th id="recent-transactions-description">{"Description"}</th>
                        <th id="recent-transactions-category">{"Category"}</th>
                        <th id="recent-transactions-date">{"Date"}</th>
                        <th id="recent-transactions-amount">{"Amount"}</th>
                    </thead>
                    <tbody id="recent-transactions-body">
                        <tr>
                            <td>{"This is a sample description for a random transaction"}</td>
                            <td>{"NormalExpenses"}</td>
                            <td>{"2023-11-07"}</td>
                            <td>{"40500.99"}</td>
                        </tr>
                        <tr>
                            <td>{"This is a sample description for a random transaction"}</td>
                            <td>{"NormalExpenses"}</td>
                            <td>{"2023-11-07"}</td>
                            <td>{"40500.99"}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        true
    }
}
