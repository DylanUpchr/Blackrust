use yew::prelude::*;
use stylist::css;

pub struct Hostname;

impl Component for Hostname {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{"hostname"}</div>
        }
    }
}