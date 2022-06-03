use yew::prelude::*;
use stylist::css;

pub struct Banner;

impl Component for Banner {
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
            background-color: gray;
            width: 100%;
            height: 150px;
            margin-top: 15px;
        ");
        html! {
            <div {class}>
            </div>
        }
    }
}