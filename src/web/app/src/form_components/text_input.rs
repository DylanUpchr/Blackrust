use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use stylist::css;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub value: String,
    pub disabled: bool,
    //pub handle_onchange: Callback<String>
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let valid = use_state(|| true);
    /*let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
            handle_onchange.emit(value);
    });*/
    html! {
        <span>
            <label for={props.name.clone()}>{format!("{}: ", props.name.clone())}</label>
            <input type="text" 
                name={props.name.clone()} 
                //{onchange} 
                disabled={props.disabled} 
                placeholder={props.name.clone()}
                value={props.value.clone()}
            />
        </span>
    }
}
