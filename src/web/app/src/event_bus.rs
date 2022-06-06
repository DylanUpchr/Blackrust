use yew_agent::{Agent, AgentLink, Context, HandlerId};
use std::collections::HashSet;
use crate::components::{ tabs::TabBar, app::AppRoute };

pub enum EventBusIOMsg {
    AddTab(u32, String, u16, AppRoute),
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for EventBus {
    type Input = EventBusIOMsg;
    type Message = ();
    type Output = EventBusIOMsg;
    type Reach = Context<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { 
            link,
            subscribers: HashSet::new()
        }
    }

    fn update(&mut self, _msg: Self::Message) {
        // no messaging
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            EventBusIOMsg::AddTab(id, name, rfb_port, route) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(
                        *sub, 
                        EventBusIOMsg::AddTab(id.clone(), name.clone(), rfb_port.clone(), route.clone())
                    );
                }       
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}