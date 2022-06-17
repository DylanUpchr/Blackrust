use yew::{ prelude::*, html::Scope };
use stylist::css;

use crate::lib::NetworkManagerProfile;
use super::{ NetSettings, NetSettingsMsg };

#[derive(Properties, PartialEq)]
pub struct ProfileOptionProps {
    pub profile: NetworkManagerProfile,
    pub selected: bool
}

pub enum ProfileOptionMsg {
    LoadProfile
}

pub struct ProfileOption {
    parent: Option<Scope<NetSettings>>
}

impl Component for ProfileOption {
    type Message = ProfileOptionMsg;
    type Properties = ProfileOptionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let parent: Option<Scope<NetSettings>>;
        let parent_link = ctx.link().get_parent();
        match parent_link {
            Some(net_settings) => {
                parent = Some(net_settings.clone().downcast::<NetSettings>());
            },
            None => parent = None
        }
        Self {
            parent: parent
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProfileOptionMsg::LoadProfile => {
                match &self.parent {
                    Some(parent) => parent.send_message(NetSettingsMsg::LoadProfile { id: ctx.props().profile.uuid.clone() }),
                    None => ()
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| ProfileOptionMsg::LoadProfile);
        let selected = ctx.props().selected;
        html! {
            <option 
                {selected} 
                {onclick}
            >
                {ctx.props().profile.name.clone()}
            </option>
        }
    }
}