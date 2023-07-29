use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};

#[function_component(ListComponent)]
pub fn list_component() -> Html {
    let name_list = use_state_eq(Vec::<String>::new);
    let input_ref = NodeRef::default();

    let on_submit = {
        let name_list = name_list.clone();
        let input_ref = input_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let mut names = (*name_list).clone();
            let name_input_element = input_ref.cast::<HtmlInputElement>().unwrap();
            let new_name = name_input_element.value();

            if new_name.is_empty() {
                return;
            }

            name_input_element.set_value("");
            names.push(new_name);
            name_list.set(names);
        })
    };

    let display_names: VNode = (*name_list)
        .iter()
        .map(|name| html! {<li>{name}</li>})
        .collect();

    html! {
        <div>
            <div>
            {"List component"}
            </div>
            <div>
                <form onsubmit={on_submit}>
                    <input type="text" ref={input_ref} placeholder="input a name"/>
                    <input type="submit" hidden=true value="Add to list"/>
                    <button type="submit">{"Add to list"}</button>
                </form>
            </div>
            <ul>
                {display_names}
            </ul>
        </div>
    }
}
