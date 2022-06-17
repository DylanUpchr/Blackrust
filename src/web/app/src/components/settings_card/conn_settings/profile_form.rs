use yew::{ prelude::*, html::Scope };
use stylist::css;

use crate::lib::Profile;
use crate::form_components::text_input::TextInput;
use super::{ ConnSettings, ConnSettingsMsg };

#[derive(Properties, PartialEq)]
pub struct ProfileFormProps {
    pub profile: Option<Profile>
}

pub enum ProfileFormMsg {
    SaveProfile { profile: Profile },
    DeleteProfile
}

pub struct ProfileForm {
    parent: Option<Scope<ConnSettings>>
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
            parent: parent
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProfileFormMsg::SaveProfile { profile } => {
                match &self.parent {
                    Some(parent) => parent.send_message(ConnSettingsMsg::SaveProfile { profile: profile.clone() }),
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

        let disabled = &ctx.props().profile.is_none();

        let profile = match &ctx.props().profile {
            Some(profile) => profile.clone(),
            None => Profile::default()
        };

        //let validator = || true;

        let handle_onchange = Callback::from(|x| 
            { 
                log::info!("{}", x);
            }
        );

        html! { 
            <form>
                /*{ 
                    match &ctx.props().profile {
                        Some(profile) => profile.name.clone(),
                        None => String::new()
                    } 
                }*/
                //<TextInput name="Profile ID" value={profile.id} disabled={true} {handle_onchange.clone()} />
                <TextInput name="Profile Name" value={profile.name} disabled={disabled.clone()} {handle_onchange} />
            </form> 
        }
    }
}