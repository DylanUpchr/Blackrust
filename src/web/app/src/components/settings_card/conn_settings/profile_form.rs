use yew::{ prelude::*, html::{ Scope, NodeRef } };
use stylist::css;
use web_sys::{ HtmlFormElement, HtmlCollection, HtmlInputElement };
use wasm_bindgen::JsCast;

use crate::lib::{ Profile, PortProtocol, NetworkManagerProfile };
use crate::form_components::{ text_input::TextInput, select_input::SelectInput };
use super::{ ConnSettings, ConnSettingsMsg };

#[derive(Properties, PartialEq)]
pub struct ProfileFormProps {
    pub profile: Option<Profile>,
    pub net_profiles: Vec<NetworkManagerProfile>
}

pub enum ProfileFormMsg {
    ModifyProfile { form: HtmlFormElement },
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
            profile: ctx.props().profile.clone()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProfileFormMsg::ModifyProfile { form } => {
                //self.profile = Some(profile.clone());
                log::info!("{:?}", form);

                let name_value = form.get_with_name("Profile Name")
                    .unwrap()
                    .unchecked_into::<HtmlInputElement>()
                    .value();

                let ip_fqdn_value = form.get_with_name("IP Address / FQDN")
                    .unwrap()
                    .unchecked_into::<HtmlInputElement>()
                    .value();

                let extra_settings_value = form.get_with_name("Extra arguments")
                    .unwrap()
                    .unchecked_into::<HtmlInputElement>()
                    .value();

                
                let network_select = form.get_with_name("Extra arguments")
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .selected_options();

                let mut selected_network_ids: Vec<String> = vec![];
                for i in 0..value.length(){
                    let selected_option = value.item(i).unwrap();
                    selected_ids.push(selected_option.get_attribute("value").unwrap())
                }

                log::info!("{:?}", name_value);

                false
            },
            ProfileFormMsg::SaveProfile => {
                match &self.parent {
                    Some(parent) => {
                        match &ctx.props().profile {
                            Some(profile) => parent.send_message(ConnSettingsMsg::SaveProfile { profile: profile.clone() }),
                            None => ()
                        }
                    },
                    None => ()
                }
                false
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
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let disabled = &ctx.props().profile.is_none();

        let profile = match &ctx.props().profile {
            Some(profile) => profile.clone(),
            None => Profile::default()
        };

        //let validator = || true;

        /*let input_handle_onchange = Callback::from(|x: String| 
            { 
                log::info!("{}", x);
            }
        );*/

        let link = ctx.link().clone();
        let form = self.form_node_ref.cast::<HtmlFormElement>();

        let onsubmit = Callback::from(move |e: FocusEvent| 
            { 
                e.prevent_default();
                let form = form
                    .clone()
                    .unwrap();

                link.send_message(ProfileFormMsg::ModifyProfile { form })
            }
        );
        let delete_onclick = ctx.link().callback(|_| ProfileFormMsg::DeleteProfile);

        html! { 
            <form {onsubmit} >
                <span>{"Profile ID: "}{profile.id}</span><br/>
                <TextInput 
                    name="Profile Name" 
                    value={profile.name} 
                    disabled={disabled.clone()} 
                    //handle_onchange={input_handle_onchange.clone()} 
                /><br/>
                //ConnectionSettings values
                <fieldset>
                    <legend>{"Connection settings"}</legend>
                    <TextInput 
                        name="IP Address / FQDN" 
                        value={profile.connection_settings.ip_fqdn} 
                        disabled={disabled.clone()} 
                        //handle_onchange={input_handle_onchange.clone()} 
                    /><br/>
                    //Protocol
                    <TextInput 
                        name="Extra arguments" 
                        value={profile.connection_settings.extra_settings} 
                        disabled={disabled.clone()} 
                        //handle_onchange={input_handle_onchange.clone()} 
                    /><br/>
                </fieldset>
                <fieldset>
                    <legend>{"Network profiles"}</legend>
                    //NetworkManagerProfile Select
                        <SelectInput 
                        values={
                            ctx.props().net_profiles.iter().map(|net_profile| 
                                (net_profile.uuid.clone(), net_profile.name.clone())
                            ).collect::<Vec<(String, String)>>()
                        }
                        selected_values={
                            profile.network_profiles.iter().map(|net_profile| 
                                (net_profile.uuid.clone(), net_profile.name.clone())
                            ).collect::<Vec<(String, String)>>()
                        }
                        disabled={disabled.clone()} 
                        //handle_onchange={select_handle_onchange.clone()} 
                    />
                </fieldset>
                <button type={"submit"} >{"Save"}</button>
                <button onclick={delete_onclick}>{"Delete"}</button>
            </form> 
        }
    }
}