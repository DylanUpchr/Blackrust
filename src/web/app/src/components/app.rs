use yew::prelude::*;
use yew_router::prelude::*;
use stylist::css;

use crate::components::{tabs::TabBar, main_card::MainCard, settings_card::SettingsCard, session_page::SessionPage};

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Index,
    #[at("/settings/*")]
    Settings,
    #[at("/session_page/:session_id")]
    Session { session_id: u32 }
}

#[derive(Clone, Routable, PartialEq)]
pub enum SettingsRoute {
    #[at("/settings/net")]
    NetworkProfiles,
    #[at("/settings/conn")]
    ConnectionProfiles,
    #[at("/settings/theme")]
    ThemeSettings,
    #[at("/settings/i18n")]
    I18nSettings,
    #[at("/settings/about")]
    About,
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Index => {
            html! { <MainCard /> }
        },
        AppRoute::Settings => {
            html! { <SettingsCard /> }
        },
        AppRoute::Session { session_id } => {
            html! { <SessionPage session_id={session_id.clone()} /> }
        }
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = 
        css!("
            height: 100%; 
            width: 100%; 
            background-color: gray;
            position: absolute;
        ");
        html! {
            <div {class}>
                <BrowserRouter>
                    <TabBar />
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        }
    }
}