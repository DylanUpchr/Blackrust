use yew::{ prelude::*, html::Scope };
use stylist::css;

use crate::lib::Profile;
use super::{ ConnSettings, ConnSettingsMsg };

#[derive(Properties, PartialEq)]
pub struct ProfileFormProps {
    pub profile: Option<Profile>
}

pub enum ProfileFormMsg {
    SaveProfile,
    DeleteProfile
}

pub struct ProfileForm {
    parent: Option<Scope<ConnSettings>>,
    profile: Option<Profile>
}

impl Component for ProfileForm {
    type Message = ProfileFormMsg;
    type Properties = ProfileFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        let parent: Option<Scope<ConnSettings>>;
        let parent_link = ctx.link().get_parent();
        match parent_link {
            Some(conn_settings) => {
                parent = Some(conn_settings.clone().downcast::<ConnSettings>());
            },
            None => parent = None
        }
        Self {
            parent: parent,
            profile: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProfileFormMsg::SaveProfile => {
                match &self.parent {
                    Some(parent) => {
                        match &self.profile {
                            Some(profile) => parent.send_message(ConnSettingsMsg::SaveProfile { profile: profile.clone() }),
                            None => ()
                        }
                    },
                    None => ()
                }
            },
            ProfileFormMsg::DeleteProfile => {
                match &self.parent {
                    Some(parent) => {
                        match &ctx.props().profile {
                            Some(profile) => parent.send_message(ConnSettingsMsg::DeleteProfile { id: profile.id.clone() }),
                            None => ()
                        }
                    },
                    None => ()
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! { 
            <form>
                { 
                    match &ctx.props().profile {
                        Some(profile) => profile.name.clone(),
                        None => String::new()
                    } 
                } 
            </form> 
        }
    }
}