use yew::{ prelude::*, html::Scope, NodeRef, events };
use yew_feather::{ chevron_up::ChevronUp, chevron_down::ChevronDown };
use stylist::css;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

mod profile_option;

use profile_option::ProfileOption;

use crate::lib::*;
use super::{ ConnectionForm, ConnectionFormMsg };

#[derive(Properties, PartialEq)]
pub struct SearchDropdownProps {
    pub selected_profile: Option<Profile>
}

pub enum SearchDropdownMsg {
    ChangeOpenState,
    LoadProfile { profile: Profile },
    GetAllProfiles,
    QueryProfiles { query: String },
    UpdateProfiles { profiles: Profiles }
}

pub struct SearchDropdown {
    parent: Option<Scope<ConnectionForm>>,
    is_open: bool,
    queried_profiles: Vec<Profile>,
    input_ref: NodeRef,
}

impl Component for SearchDropdown {
    type Message = SearchDropdownMsg;
    type Properties = SearchDropdownProps;

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
            queried_profiles: vec![],
            input_ref: NodeRef::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchDropdownMsg::ChangeOpenState => {
                if self.is_open {
                    self.is_open = false;
                } else {
                    self.is_open = true;
                }
                true
            }
            SearchDropdownMsg::LoadProfile { profile } => {
                match &self.parent {
                    Some(parent) => parent.send_message(ConnectionFormMsg::LoadProfile { profile }),
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
                let dropdown_is_open = self.is_open;
                if query != "" {
                    spawn_local(async move {
                        match query_profiles(query).await {
                            Ok(profiles) => { 
                                link.send_message(SearchDropdownMsg::UpdateProfiles { profiles });
                                if !dropdown_is_open {
                                    link.send_message(SearchDropdownMsg::ChangeOpenState);
                                }
                            },
                            Err(_) => ()
                        } 
                    });
                }
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

        let link = ctx.link();

        let onclick = link.callback(|_| SearchDropdownMsg::ChangeOpenState);

        let icon = if self.is_open { html! {<ChevronUp size="20"/>} } else { html! {<ChevronDown size="20"/>}};

        let profiles = &self.queried_profiles;

        let input_ref = self.input_ref.clone();

        let parent = self.parent.clone();

        let oninput = link.batch_callback(move |e: InputEvent| {
            match &parent {
                Some(parent) => parent.send_message(ConnectionFormMsg::DeselectProfile),
                None => ()
            }
            let input = input_ref.cast::<HtmlInputElement>();
            input.map(|input| SearchDropdownMsg::QueryProfiles { query: input.value() })
        });

        let value = match &ctx.props().selected_profile {
            Some(profile) => format!("{}", profile.name.clone()),
            None => {
                let input_ref = self.input_ref.clone();
                let input = input_ref.cast::<HtmlInputElement>();
                match input.map(|input| input.value() ) {
                    Some(value) => value,
                    None => String::new()
                }
            }
        };

        html! {
            <span>
                <div class={input_container_class}>
                    <input ref={self.input_ref.clone()}
                        class={classes!(input_class.clone(), text_input_class)} type="text" 
                        {oninput}
                        {value}
                    />
                    <button class={classes!(input_class, button_class)} {onclick}>
                        {icon}
                    </button>
                </div>
                <div class={profile_option_list_class}>
                    { 
                        profiles.into_iter().map(|profile| {
                            let selected = match &ctx.props().selected_profile {
                                Some(selected_profile) => {
                                    profile == selected_profile
                                },
                                None => false
                            };
                            html! { <ProfileOption {profile} {selected} />}
                        }).collect::<Html>()
                    }
                </div>
            </span>
        }
    }
}

async fn query_profiles(query: String) -> Result<Profiles, ()>{
    let call = Request::get(&format!("/cfg_mgr/profiles/{}", query))
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