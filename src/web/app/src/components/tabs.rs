use yew::prelude::*;
use yew_router::prelude::*;
use stylist::css;

use crate::components::app::AppRoute;

pub enum Msg {
    AddTab {id: u32, name: String, rfb_port:u16},
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
    rfb_port: u16
}

pub struct TabBar {
    tabs: Vec<Tab>
}

pub struct Tab {
    id: u32,
    name: String,
    rfb_port: u16,
    active_tab: bool
}

impl Component for TabBar {
    type Message = Msg;
    type Properties = TabBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            tabs: vec![]
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTab {id, name, rfb_port} => {
                let tab = Tab {
                    id: id,
                    name: String::from("name"),
                    rfb_port: 0,
                    active_tab: true
                };
                self.tabs.push(tab);
                true
            },
            Msg::RemoveTab => {
                true
            },
            _ => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let class = 
        css!("
            display: flex;
            flex-direction: row;
            background-color: white;
            border-radius: 10px;
            margin: 15px;
            padding: 10px;
        ");
        let onclick = link.callback(|_| Msg::AddTab {id: 0, name: String::from("test"), rfb_port: 0});
        let tabs = &self.tabs;
        html! {
            <nav id="tabBar" {class}>
                <span>{"tabbar nbTabs: "}{ self.tabs.len()}</span>
                { 
                    tabs.into_iter().map(|tab| {
                        html!{ <Tab id={tab.id} name={tab.name.clone()} rfb_port={tab.rfb_port}/>}
                    }).collect::<Html>()
                }
                <button {onclick}>{ "New tab" }</button>
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
            active_tab: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let class = css!("background-color: gray;");
        html! {
            <div {class}>
                <span>
                {"tab n° "}
                {ctx.props().id}
                {" named: "}
                {ctx.props().name.clone()}
                <Link<AppRoute> to={AppRoute::Session {session_id: ctx.props().id}}>{ "click here to go session" }</Link<AppRoute>>
                </span>
            </div>
        }
    }
}