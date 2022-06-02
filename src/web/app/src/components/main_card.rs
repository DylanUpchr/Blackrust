use yew::prelude::*;
use yew_router::prelude::Link;
use crate::components::app::AppRoute;

pub struct MainCard;

impl Component for MainCard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{"test index"}</p>
                <Link<AppRoute> to={AppRoute::Settings}>{ "click here to go settings" }</Link<AppRoute>>
            </>
        }
    }
}