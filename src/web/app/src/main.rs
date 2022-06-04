use yew::prelude::*;
use stylist::yew::styled_component;

mod components;
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
    yew::start_app::<Root>();
}