use yew::prelude::*;
use stylist::css;

mod hostname;
use hostname::Hostname;

mod time;
mod settings_button;

use time::Time;
use settings_button::SettingsButton;

#[derive(Properties, PartialEq)]
pub struct TopBarProps {
    pub settings_open: bool
}

pub struct TopBar;

impl Component for TopBar {
    type Message = ();
    type Properties = TopBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!("
            width: 100%;
            height: 3%;
            padding-top: 5px;
            padding-bottom: 5px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            border-bottom: 1px solid lightgray;
        ");
        html! {
            <div {class}>
                <Hostname />
                <Time />
                <SettingsButton open={ctx.props().settings_open}/>
            </div>
        }
    }
}