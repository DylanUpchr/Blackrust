use yew::prelude::*;
use yew_router::prelude::Link;
use yew_feather::sliders::Sliders;
use stylist::css;
use crate::components::app::AppRoute;

mod hostname;
use hostname::Hostname;

mod time;
use time::Time;

pub struct TopBar;

impl Component for TopBar {
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
            height: 3%;
            display: flex;
            align-items: center;
            justify-content: space-between;
        ");
        html! {
            <div {class}>
                <Hostname />
                <Time />
                <Link<AppRoute> to={AppRoute::SettingsRoot}><Sliders/></Link<AppRoute>>
            </div>
        }
    }
}