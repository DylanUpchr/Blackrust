use yew::prelude::*;
use stylist::css;

pub struct ThemeSettings;

impl Component for ThemeSettings {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let class = css!("
            display: inline-block;
            width: 84%;
            height: 100%;
            margin-top: 15px;
            float: right;
        ");
        html! {
            <div {class}>
                {"theme"}
            </div>
        }
    }
}