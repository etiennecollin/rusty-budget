use web_sys::{HtmlInputElement};
use yew::{html, Component, Html, InputEvent, NodeRef, Properties, TargetCast};

pub enum Msg {
    SetInput(u32),
    AddValue(u32),
    DoNothing,
}

#[derive(Properties, PartialEq)]
pub struct TestComponentProps {
    pub min_value: u32,
    pub max_value: u32,
}

pub struct TestComponent {
    input_ref: NodeRef,
    value_list: Vec<u32>,
}

impl Component for TestComponent {
    type Message = Msg;
    type Properties = TestComponentProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
            value_list: Vec::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let min_val = ctx.props().min_value;
        let max_val = ctx.props().max_value;

        let on_click = {
            let input_ref = self.input_ref.clone();

            ctx.link().callback(move |_e| {
                let input_element = input_ref.cast::<HtmlInputElement>().unwrap();
                let input_value = input_element.value();

                input_element.set_value("");
                match input_value.parse() {
                    Ok(val) => Msg::AddValue(val),
                    Err(_) => {
                        log::error!("error ocurred parsing value: {}", input_value);
                        Msg::DoNothing
                    }
                }
            })
        };

        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input_element: HtmlInputElement = e.target_unchecked_into();

            let mut val: u32 = match input_element.value().parse() {
                Ok(val) => val,
                Err(err) => {
                    log::error!("error ocurred parsing value: {}", err);
                    0
                }
            };

            log::debug!("Input value: {}", val);
            if val < min_val {
                val = min_val;
            } else if val > max_val {
                val = max_val;
            }

            Msg::SetInput(val)
        });

        let display_values = self.value_list.iter().map(|value| html! {<li>{value}</li>});

        html! {
            <div>
                <div>
                    <label>{format!("Input value. Min: {}, Max: {}", min_val, max_val)}</label>
                    <input ref={self.input_ref.clone()} type="number" placeholder="input a number" oninput={on_input} min={min_val.to_string()} max={max_val.to_string()}/>
                    <button onclick={on_click}>{"Add to list"}</button>
                </div>
                <div>
                    <ul>{for display_values}</ul>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInput(val) => {
                log::debug!("Setting input value to: {}", val);
                let input_element = self.input_ref.clone().cast::<HtmlInputElement>().unwrap();
                input_element.set_value(&val.to_string());
                true
            }
            Msg::AddValue(val) => {
                log::debug!("Adding value to list: {}", val);
                self.value_list.push(val);
                true
            }
            Msg::DoNothing => {
                log::debug!("Doing nothing");
                false
            }
        }
    }
}
