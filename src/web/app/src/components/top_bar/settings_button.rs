use yew::prelude::*;
use yew_feather::{ sliders::Sliders, x_circle::XCircle };
use yew_router::prelude::Link;
use crate::components::app::AppRoute;

pub struct SettingsButton;

#[derive(Properties, PartialEq)]
pub struct SettingsButtonProps {
    pub open: bool
}

impl Component for SettingsButton {
    type Message = ();
    type Properties = SettingsButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let app_route: AppRoute;
        let icon: Html;
        if ctx.props().open {
            app_route = AppRoute::Index;
            icon = html! { <XCircle /> }
        } else {
            app_route = AppRoute::SettingsRoot;
            icon = html! { <Sliders /> }
        }
        html! {
            <div>
                <Link<AppRoute> to={app_route}>
                    {icon}
                </Link<AppRoute>>
            </div>
        }
    }
}