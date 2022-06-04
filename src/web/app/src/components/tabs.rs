use yew::prelude::*;
use yew_router::prelude::*;
use stylist::css;

use crate::components::app::AppRoute;

pub enum Msg {
    AddTab {id: u32, name: String, rfb_port: u16, route: AppRoute},
    RemoveTab
}

#[derive(Properties, PartialEq)]
pub struct TabBarProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Properties, PartialEq)]
pub struct TabProps {
    #[prop_or_default]
    id: u32,
    name: String,
    rfb_port: u16,
    route: AppRoute,
    is_active_tab: bool
}

pub struct TabBar {
    tabs: Vec<Tab>
}

pub struct Tab {
    id: u32,
    name: String,
    rfb_port: u16,
    route: AppRoute,
    is_active_tab: bool
}

impl Component for TabBar {
    type Message = Msg;
    type Properties = TabBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            tabs: vec![
                Tab {
                    id: 0,
                    name: "Home".to_string(),
                    rfb_port: 0,
                    route: AppRoute::Index,
                    is_active_tab: true
                }
            ]
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTab {id, name, rfb_port, route} => {
                let tab = Tab {
                    id: id,
                    name: String::from(name),
                    rfb_port: 0,
                    route: AppRoute::Session {session_id: id},
                    is_active_tab: true
                };
                self.tabs.push(tab);
                true
            },
            Msg::RemoveTab => {
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let class = 
        css!("
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;
            background-color: white;
            border-radius: 10px;
            margin: 15px;
            padding: 5px;
        ");
        let tabs = &self.tabs;
        html! {
            <nav id="tabBar" {class}>
                { 
                    tabs.into_iter().map(|tab| {
                        html!{ 
                            <Tab 
                                id={ tab.id } 
                                name={ tab.name.clone() } 
                                rfb_port={ tab.rfb_port } 
                                route={ tab.route.clone() }
                                is_active_tab = { tab.is_active_tab }
                            />
                        }
                    }).collect::<Html>()
                }
            </nav>
        }
    }
}

impl Component for Tab {
    type Message = ();
    type Properties = TabProps;
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: 0,
            name: String::from("Session tab"),
            rfb_port: 0,
            route: AppRoute::Index,
            is_active_tab: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!(
            r#"
            padding: 10px;
            border: 1px solid ${accent};
            border-radius: 5px;
            margin: 10px;

            a {
                text-decoration: none;
                color: black;
                
            }
            "#, 
            accent = if ctx.props().is_active_tab { "blue" } else { "black" }
        );
        html! {
            <div {class}>
                <Link<AppRoute> to={ctx.props().route.clone()}>{ ctx.props().name.clone() }</Link<AppRoute>>
            </div>
        }
    }
}