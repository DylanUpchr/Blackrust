use yew::prelude::*;
use stylist::css;
use yew_router::prelude::{ Switch, Redirect, Link };
use yew_feather::{
    bar_chart::BarChart,
    user::User,
    droplet::Droplet,
    globe::Globe,
    info::Info
};
use crate::components::app::SettingsRoute;
use crate::components::top_bar::TopBar;

mod net_settings;
mod conn_settings;
mod theme_settings;
mod i18n_settings;
mod about;

use net_settings::NetSettings;
use conn_settings::ConnSettings;
use theme_settings::ThemeSettings;
use i18n_settings::I18nSettings;
use about::About;

pub struct SettingsCard;

impl Component for SettingsCard {
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
        ");

        let sidebar_class = css!("
            display: inline-block;
            height: 100%;
            width: 15%;
            border-right: 1px solid lightgray;

            ul {
                list-style-type: none;
                padding: 0px;
            }

            ul li {
                margin-top: 25px;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                width: 100%;
            }

            ul li a {
                text-decoration: none;
                color: black;
                font-size: 1.2rem;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
            }
        ");

        html! {
            <div {class}>
                <TopBar settings_open=true/>
                <div class={sidebar_class}>
                    <ul>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::NetworkProfiles}>
                                <BarChart color="black" size="64" /><br/>
                                {"Network profile"}
                            </Link<SettingsRoute>>
                        </li>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::ConnectionProfiles}>
                                <User color="black" size="64" /><br/>
                                {"Connection profile"}
                            </Link<SettingsRoute>>
                        </li>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::ThemeSettings}>
                                <Droplet color="black" size="64" /><br/>
                                {"Theme settings"}
                            </Link<SettingsRoute>>
                        </li>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::I18nSettings}>
                                <Globe color="black" size="64" /><br/>
                                {"Language / Locale settings"}
                            </Link<SettingsRoute>>
                        </li>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::About}>
                                <Info color="black" size="64" /><br/>
                                {"About"}
                            </Link<SettingsRoute>>
                        </li>
                    </ul>
                </div>
                <Switch<SettingsRoute> render={Switch::render(switch_settings)} />
            </div>
        }
    }
}

fn switch_settings(routes: &SettingsRoute) -> Html {
    match routes {
        SettingsRoute::DefaultRoute => {
            html! { <Redirect<SettingsRoute> to={SettingsRoute::NetworkProfiles}/> }
        },
        SettingsRoute::NetworkProfiles => {
            html! { <NetSettings /> }
        },
        SettingsRoute::ConnectionProfiles => {
            html! { <ConnSettings /> }
        },
        SettingsRoute::ThemeSettings => {
            html! { <ThemeSettings /> }
        },
        SettingsRoute::I18nSettings => {
            html! { <I18nSettings /> }
        },
        SettingsRoute::About => {
            html! { <About /> }
        }
    }
}