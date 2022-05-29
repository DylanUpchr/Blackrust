use yew::prelude::*;

pub enum Msg {
    AddTab,
    RemoveTab
}

#[derive(Properties, PartialEq)]
pub struct TabBarProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct TabBar {
    tabs: Vec<Tab>
}

pub struct Tab {
    id: u32
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
            Msg::AddTab => {
                self.tabs.push(
                    Tab {
                        id: self.tabs.len().try_into().unwrap()
                    }
                );
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
        let onclick = link.callback(|_| Msg::AddTab);
        let tabs = &self.tabs;
        html! {
            <div>
                <p>{"tabbar nbTabs: "}{ self.tabs.len()}</p>
                { 
                    tabs.into_iter().map(|tab| {
                        html!{<div>{"tab number "} {tab.id}</div>}
                    }).collect::<Html>()
                }
                <button {onclick}>{ "New tab" }</button>
            </div>
        }
    }
}

impl Component for Tab {
    type Message = ();
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <p>{"tab"}</p>
            </div>
        }
    }
}