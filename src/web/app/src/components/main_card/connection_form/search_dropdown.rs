use yew::{ prelude::*, html::Scope };
use yew_feather::{ chevron_up::ChevronUp, chevron_down::ChevronDown };
use stylist::css;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

mod profile_option;

use profile_option::ProfileOption;

use crate::lib::*;
use super::{ ConnectionForm, ConnectionFormMsg };

pub struct SearchDropdown {
    parent: Option<Scope<ConnectionForm>>,
    is_open: bool,
    queried_profiles: Vec<Profile>
}

pub enum SearchDropdownMsg {
    ChangeOpenState,
    LoadProfile { id: String },
    GetAllProfiles,
    QueryProfiles { query: String },
    UpdateProfiles { profiles: Profiles }
}

impl Component for SearchDropdown {
    type Message = SearchDropdownMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(SearchDropdownMsg::GetAllProfiles);
        let parent: Option<Scope<ConnectionForm>>;
        let parent_link = ctx.link().get_parent();
        match parent_link {
            Some(connection_form) => {
                parent = Some(connection_form.clone().downcast::<ConnectionForm>());
            },
            None => parent = None
        }

        Self {
            parent: parent,
            is_open: false,
            queried_profiles: vec![]
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchDropdownMsg::ChangeOpenState => {
                if self.is_open {
                    self.is_open = false;
                } else {
                    self.is_open = true;
                    ctx.link().send_message(SearchDropdownMsg::GetAllProfiles);
                }
                true
            }
            SearchDropdownMsg::LoadProfile { id } => {
                match &self.parent {
                    Some(parent) => parent.send_message(ConnectionFormMsg::LoadProfile { id }),
                    None => ()
                }
                true
            },
            SearchDropdownMsg::GetAllProfiles => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match get_all_profiles().await {
                        Ok(profiles) => link.send_message(SearchDropdownMsg::UpdateProfiles { profiles }),
                        Err(_) => ()
                    } 
                });
                false
            },
            SearchDropdownMsg::QueryProfiles { query } => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match query_profiles(query).await {
                        Ok(profiles) => link.send_message(SearchDropdownMsg::UpdateProfiles { profiles }),
                        Err(_) => ()
                    } 
                });
                false
            }
            SearchDropdownMsg::UpdateProfiles { profiles } => {
                self.queried_profiles = profiles.profile_vec;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_container_class = css!("
            border: 1px solid gray;
            display: flex;
        ");

        let profile_option_list_class = css!(r#"
            width: 100%;
            height: auto;
            display: ${display};
        "#,
        display = if self.is_open { "block" } else { "none" }
        );

        let input_class = css!("
            border: none;
            height: 20px;
        ");

        let button_class = css!("
            border-left: 1px solid gray;
        ");

        let text_input_class = css!("
            width: 500px;
        ");

        let onclick = ctx.link().callback(|_| SearchDropdownMsg::ChangeOpenState);

        let icon = if self.is_open { html! {<ChevronUp size="20"/>} } else { html! {<ChevronDown size="20"/>}};

        let profiles = &self.queried_profiles;

        html! {
            <span>
                <div class={input_container_class}>
                    <input class={classes!(input_class.clone(), text_input_class)} type="text" />
                    <button class={classes!(input_class, button_class)} {onclick}>
                        {icon}
                    </button>
                </div>
                <div class={profile_option_list_class}>
                    { 
                        profiles.into_iter().map(|profile| {
                            html! { <ProfileOption {profile} />}
                        }).collect::<Html>()
                    }
                </div>
            </span>
        }
    }
}

async fn query_profiles(query: String) -> Result<Profiles, ()>{
    let call = Request::get(&format!("/cfg_mgr/profile/{}", query))
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

async fn get_all_profiles() -> Result<Profiles, ()>{
    let call = Request::get("/cfg_mgr/profiles")
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