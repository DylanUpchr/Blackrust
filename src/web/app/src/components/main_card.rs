use yew::prelude::*;
use yew_router::prelude::Link;
use stylist::css;
use crate::components::{ app::AppRoute, top_bar::TopBar };

pub struct MainCard;

impl Component for MainCard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!("
            background-color: white;
            border-radius: 10px;
            margin: 15px;
            padding: 10px;
        ");

        html! {
            <div {class}>
                <TopBar/>
            </div>
        }
    }
}