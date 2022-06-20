use yew::prelude::*;
use stylist::css;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::lib::*;

mod profile_option;
mod profile_form;

use profile_option::ProfileOption;
use profile_form::ProfileForm;

pub struct ConnSettings {
    profiles: ( Profiles, Vec<NetworkManagerProfile> ),
    selected_profile: Option<Profile>
}

pub enum ConnSettingsMsg {
    GetAllProfiles,
    UpdateProfiles { profiles: ( Profiles, Vec<NetworkManagerProfile> ) },
    LoadProfile { id: String },
    SaveProfile { profile: Profile },
    CreateProfile,
    DeleteProfile { id: String }
}

impl Component for ConnSettings {
    type Message = ConnSettingsMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(ConnSettingsMsg::GetAllProfiles);
        Self {
            profiles: (Profiles::new(), vec!()),
            selected_profile: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ConnSettingsMsg::GetAllProfiles => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match get_all_profiles().await {
                        Ok(profiles) => link.send_message(ConnSettingsMsg::UpdateProfiles { profiles }),
                        Err(_) => ()
                    } 
                });
                false
            },
            ConnSettingsMsg::UpdateProfiles { profiles } => {
                self.profiles = profiles;
                true
            },
            ConnSettingsMsg::LoadProfile { id } => {
                self.selected_profile = self.profiles.0.profile_vec.iter().find(|profile| profile.id == id).cloned();
                true
            },
            ConnSettingsMsg::SaveProfile { profile } => {
                false
            },
            ConnSettingsMsg::CreateProfile => {
                false
            },
            ConnSettingsMsg::DeleteProfile { id } => {
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

        let profiles = &self.profiles.0.profile_vec;
        let onclick = ctx.link().callback(|_| ConnSettingsMsg::CreateProfile);

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
                    <ProfileForm 
                        profile={ self.selected_profile.clone() }
                        net_profiles={ self.profiles.1.clone() }
                    />
                </div>
            </div>
        }
    }
}

async fn get_all_profiles() -> Result<(Profiles, Vec<NetworkManagerProfile>), ()>{
    let call_cfg_profiles = Request::get("/cfg_mgr/profiles")
    .send()
    .await;

    let call_net_profiles = super::net_settings::get_all_profiles().await;

    match call_cfg_profiles {
        Ok(resp_cfg_profiles) => {
            if resp_cfg_profiles.ok() {
                match call_net_profiles {
                    Ok(resp_net_profiles) => Ok(
                        (
                            serde_json::from_str(
                                &resp_cfg_profiles.text().await.unwrap()
                            ).unwrap(),
                            resp_net_profiles
                        )
                    ),
                    Err(_) => Err(())
                }
            } else {
                Err(())
            }
        },
        Err(_) => Err(())
    }
}