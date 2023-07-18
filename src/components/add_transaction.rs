use web_sys::{HtmlInputElement, MouseEvent};
use yew::{html, Component, Html, InputEvent, NodeRef, Properties, TargetCast};

// pub enum Msg {
//     SetInput(u32),
//     AddValue(u32),
//     DoNothing,
// }

// #[derive(Properties, PartialEq)]
// pub struct AddTransactionProps {
//     pub min_value: u32,
//     pub max_value: u32,
// }

pub struct AddTransaction {
    input_ref: NodeRef,
}

impl Component for AddTransaction {
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
                <h1>{"Add Transaction"}</h1>
                <div id="add-transaction" class="col-centered">
                    <button type="button">{"Add Transaction"}</button>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        true
    }
}
