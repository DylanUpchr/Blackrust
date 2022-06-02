use yew::prelude::*;
use yew_router::prelude::{ Link, Switch };
use crate::components::app::SettingsRoute;

pub struct SettingsCard;

impl Component for SettingsCard {
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
                <p>{"Settings card"}</p>
                <Switch<SettingsRoute> render={Switch::render(switch_settings)} />
            </>
        }
    }
}

fn switch_settings(routes: &SettingsRoute) -> Html {
    match routes {
        SettingsRoute::NetworkProfiles => {
            html! { "test net" }
        },
        SettingsRoute::ConnectionProfiles => {
            html! { "test conn" }
        },
        SettingsRoute::ThemeSettings => {
            html! { "test theme" }
        },
        SettingsRoute::I18nSettings => {
            html! { "test i18n" }
        },
        SettingsRoute::About => {
            html! { "test about" }
        }
    }
}