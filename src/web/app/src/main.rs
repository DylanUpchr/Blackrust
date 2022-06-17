use yew::prelude::*;
use stylist::yew::styled_component;
use wasm_logger;

mod components;
mod form_components;
mod event_bus;
mod lib;
use components::app::App;

#[styled_component(Root)]
pub fn root() -> Html {
    html! {
        //<ThemeProvider>
            <App />
        //</ThemeProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Root>();
}