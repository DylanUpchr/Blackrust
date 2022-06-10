use yew::prelude::*;
use yew_router::prelude::*;
use stylist::css;
use yew_feather::x::X;
use yew_agent::{Bridge, Bridged};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::event_bus::{ EventBus, EventBusIOMsg };
use crate::components::app::AppRoute;

pub enum TabBarMsg {
    AddTab(u32, String, u16, AppRoute),
    RemoveTab(u32),
    ChangeTab(u32)
}

impl From<EventBusIOMsg> for TabBarMsg {
    fn from(msg: EventBusIOMsg) -> Self {
        match msg {
            EventBusIOMsg::AddTab(id, name, rfb_port, route) => {
                TabBarMsg::AddTab(id, name, rfb_port, route)
            }
        }
    }
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
    is_active_tab: bool,
    closable: bool
}

pub struct TabBar {
    tabs: Vec<Tab>,
    _producer: Box<dyn Bridge<EventBus>>
}

pub enum TabMsg {
    Disconnect
}

pub struct Tab {
    id: u32,
    name: String,
    rfb_port: u16,
    route: AppRoute,
    is_active_tab: bool,
    closable: bool
}

impl Component for TabBar {
    type Message = TabBarMsg;
    type Properties = TabBarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            tabs: vec![
                Tab {
                    id: 0,
                    name: "Home".to_string(),
                    rfb_port: 0,
                    route: AppRoute::Index,
                    is_active_tab: true,
                    closable: false
                }
            ],
            _producer: EventBus::bridge(ctx.link().callback(|val: EventBusIOMsg| val)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabBarMsg::AddTab(id, name, rfb_port, route) => {
                let tab = Tab {
                    id: id,
                    name: String::from(name),
                    rfb_port: 0,
                    route: AppRoute::Session {session_id: id},
                    is_active_tab: true,
                    closable: true
                };
                self.tabs.iter_mut().for_each(|tab| tab.is_active_tab = false);
                self.tabs.push(tab);
                true
            },
            TabBarMsg::RemoveTab(id) => {
                self.tabs.retain(|tab| tab.id != id);
                
                true
            },
            TabBarMsg::ChangeTab(id) => {                
                self.tabs.iter_mut().for_each(|tab| tab.is_active_tab = false);
                let tab = self.tabs.iter_mut().find(|tab| tab.id == id).unwrap();
                tab.is_active_tab = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                                is_active_tab={ tab.is_active_tab }
                                closable={ tab.closable }
                            />
                        }
                    }).collect::<Html>()
                }
            </nav>
        }
    }
}

impl Component for Tab {
    type Message = TabMsg;
    type Properties = TabProps;
    
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            name: ctx.props().name.clone(),
            rfb_port: ctx.props().rfb_port,
            route: ctx.props().route.clone(),
            is_active_tab: ctx.props().is_active_tab,
            closable: ctx.props().closable
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabMsg::Disconnect => {
                let parent_link = ctx.link().get_parent();
                match parent_link {
                    Some(parent_scope) => {
                        let parent = parent_scope.clone().downcast::<TabBar>();
                        parent.send_message(TabBarMsg::RemoveTab(self.id))
                    },
                    None => ()
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!(
            r#"
            padding: 10px;
            border: 1px solid ${accent};
            border-radius: 5px;
            margin: 10px;
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;

            a {
                text-decoration: none;
                color: black;
                
            }

            span:hover {
                cursor: pointer;
            }
            "#, 
            accent = if ctx.props().is_active_tab { "blue" } else { "black" }
        );

        let onclick = ctx.link().callback(|_| TabMsg::Disconnect);

        html! {
            <div {class}>
                <Link<AppRoute> to={ctx.props().route.clone()}>{ ctx.props().name.clone() }</Link<AppRoute>>
                { if self.closable { html! { <span {onclick}><X size="20" /></span> } } else { html! {}} }
            </div>
        }
    }
}

async fn disconnect_session(id: u32) {

}