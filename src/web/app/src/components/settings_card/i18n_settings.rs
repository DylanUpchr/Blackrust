use yew::prelude::*;
use stylist::css;

pub struct I18nSettings;

impl Component for I18nSettings {
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
            width: 100%;
            margin-top: 15px;
        ");
        html! {
            <div {class}>
            </div>
        }
    }
}