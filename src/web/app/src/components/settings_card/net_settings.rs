use yew::prelude::*;
use stylist::css;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::lib::NetworkManagerProfile;

mod profile_option;
mod profile_form;

use profile_option::ProfileOption;
use profile_form::ProfileForm;

pub struct NetSettings {
    profiles: Vec<NetworkManagerProfile>,
    selected_profile: Option<NetworkManagerProfile>
}

pub enum NetSettingsMsg {
    GetAllProfiles,
    UpdateProfiles { profiles: Vec<NetworkManagerProfile> },
    LoadProfile { id: String },
    SaveProfile { profile: NetworkManagerProfile },
    CreateProfile,
    DeleteProfile { id: String }
}

impl Component for NetSettings {
    type Message = NetSettingsMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(NetSettingsMsg::GetAllProfiles);
        Self {
            profiles: vec![],
            selected_profile: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NetSettingsMsg::GetAllProfiles => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match get_all_profiles().await {
                        Ok(profiles) => link.send_message(NetSettingsMsg::UpdateProfiles { profiles }),
                        Err(_) => ()
                    } 
                });
                false
            },
            NetSettingsMsg::UpdateProfiles { profiles } => {
                self.profiles = profiles;
                true
            },
            NetSettingsMsg::LoadProfile { id } => {
                self.selected_profile = self.profiles.iter().find(|profile| profile.uuid == id).cloned();
                true
            },
            NetSettingsMsg::SaveProfile { profile } => {
                false
            },
            NetSettingsMsg::CreateProfile => {
                false
            },
            NetSettingsMsg::DeleteProfile { id } => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!("
            display: flex;
            width: 84%;
            height: 600px;
            margin-top: 15px;
            float: right;
        ");
        let select_container_class = css!("
            display: inline-block;
            height: 100%;
            width: 175px;
        ");
        let select_button_class = css!("
            width: 100%;
        ");
        let select_class = css!("
            height: calc(100% - 21.5px);
            width: 100%;
        ");
        let form_container_class = css!("
            display: inline-block;
            height: calc(100% - 20px);
            margin-left: 10px;
            margin-right: 10px;
            width: calc(100% - 200px);
            border: 1px solid gray;
            border-radius: 5px;
        ");

       
        let profiles = &self.profiles;
        let onclick = ctx.link().callback(|_| NetSettingsMsg::CreateProfile);

        log::info!("net_settings: {:?}", self.selected_profile);
        html! {
            <div {class}>
                <div class={select_container_class}>
                    <select size={"5"} class={select_class}>
                    {
                        profiles.into_iter().map(|profile| {
                            let selected = match &self.selected_profile {
                                Some(selected_profile) => profile == selected_profile,
                                None => false
                            };
                            html! { <ProfileOption {profile} {selected} /> }
                        }).collect::<Html>()
                    }
                    </select>
                    <button class={select_button_class} {onclick} >{"Create profile"}</button>
                </div>
                <div class={form_container_class}>
                    <ProfileForm profile={ self.selected_profile.clone() } />
                </div>
            </div>
        }
    }
}

pub async fn get_all_profiles() -> Result<Vec<NetworkManagerProfile>, ()>{
    let call = Request::get("/net_mgr/profiles")
    .send()
    .await;

    match call {
        Ok(resp) => {
            if resp.ok() {
                Ok(
                    serde_json::from_str(
                        &resp.text().await.unwrap()
                    ).unwrap()
                )
            } else {
                Err(())
            }
        },
        Err(_) => Err(())
    }
}