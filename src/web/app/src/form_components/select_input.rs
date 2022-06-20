use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use stylist::css;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub values: Vec<(String, String)>,
    pub selected_values: Vec<(String, String)>,
    pub disabled: bool,
    //pub handle_onchange: Callback<Vec<String>>
}

#[function_component(SelectInput)]
pub fn text_input(props: &Props) -> Html {
    let valid = use_state(|| true);
    /*let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>()
            .selected_options();

        let mut selected_ids: Vec<String> = vec![];
        for i in 0..value.length(){
            let selected_option = value.item(i).unwrap();
            selected_ids.push(selected_option.get_attribute("value").unwrap())
        }

        handle_onchange.emit(selected_ids);
    });*/

    html! {
        <span>
            <select
                multiple=true
                //{onchange}
                disabled={props.disabled} 
            >
                {
                    props.values.iter().map(|option_val|
                        html! { 
                            <option 
                                selected={props.selected_values.contains(&option_val)}
                                value={option_val.0.clone()}
                            >
                                {option_val.1.clone()}
                            </option> 
                        }
                    ).collect::<Html>()
                }
            </select>
        </span>
    }
}
