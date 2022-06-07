use yew::prelude::*;
use stylist::css;
use crate::components::{ top_bar::TopBar };

mod banner;
mod connection_form;

use banner::Banner;
use connection_form::ConnectionForm;

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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let class = css!("
            background-color: white;
            border-radius: 10px;
            margin: 15px;
            padding: 10px;
            height: 780px;
        ");

        html! {
            <div {class}>
                <TopBar settings_open=false/>
                <Banner />
                <ConnectionForm />
            </div>
        }
    }
}