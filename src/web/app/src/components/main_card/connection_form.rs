use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};
use log::{info, trace, warn};
use stylist::css;
use reqwasm::http::{ Request, FormData };
use wasm_bindgen_futures::spawn_local;

use crate::lib::{ Profile, Session, ProfileFormData };
use crate::components::app::AppRoute;
use crate::event_bus::{ EventBus, EventBusIOMsg };

mod search_dropdown;

use search_dropdown::SearchDropdown;

pub struct ConnectionForm {
    event_bus: Dispatcher<EventBus>,
    selected_profile: Option<Profile>
}

pub enum ConnectionFormMsg {
    LoadProfile { profile: Profile },
    Connect,
    DeselectProfile,
    AddTab { session: Session }
}

impl Component for ConnectionForm {
    type Message = ConnectionFormMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            event_bus: EventBus::dispatcher(),
            selected_profile: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ConnectionFormMsg::LoadProfile { profile } => {
                self.selected_profile = Some(profile);
                true
            },
            ConnectionFormMsg::DeselectProfile => {
                self.selected_profile = None;
                true
            },
            ConnectionFormMsg::Connect => {
                let link = ctx.link().clone();
                let event_bus = &self.event_bus;
                let selected_profile = self.selected_profile.clone();
                match selected_profile {
                    Some(profile) => {
                        spawn_local(async move {
                            match connect(profile.clone()).await {
                                Ok(session_id) => {
                                    match get_session_by_id(session_id).await {
                                        Ok(session) => link.send_message(ConnectionFormMsg::AddTab { session }),
                                        Err(message) => log::info!("{}", message)
                                    }
                                },
                                Err(message) => log::info!("{}", message)
                            } 
                        });
                    },
                    None => ()
                }
                false
            },
            ConnectionFormMsg::AddTab { session } => {
                self.event_bus.send(
                    EventBusIOMsg::AddTab(
                        session.id, 
                        session.name.to_owned(), 
                        session.rfb_port, 
                        AppRoute::Session { session_id: session.id }
                    )
                );
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!("
            width: calc(100% - 30px);
            height: 100%;
            margin: 15px;
            display: flex;
            align-items: flex-start;
            justify-content: center;

            select, span, button {
                margin-left: 15px;
                float: top;
            }
        ");

        let select_class = css!("
            height: 20px;
        ");

        let onclick = ctx.link().callback(|_| ConnectionFormMsg::Connect);

        html! {
            <div {class}>
                <select>
                    <option selected=true>{"RDP"}</option>
                    <option>{"XDMCP"}</option>
                    <option>{"VNC"}</option>
                    <option>{"SSH - X11 Forwarding"}</option>
                </select>
                <SearchDropdown selected_profile={self.selected_profile.clone()} />
                <button {onclick}>{"Connect"}</button>
            </div>
        }
    }
}

async fn get_session_by_id(id: u32) -> Result<Session, String> {
    let call = Request::get(&format!("/rs_mgr/session/{}", id))
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
                Err(String::from("Session not found"))
            }
        },
        Err(_) => Err(String::from("Could not send request"))
    }
}

async fn connect(profile: Profile) -> Result<u32, String> {
    let body = ProfileFormData { profile };

    let call = Request::post("/rs_mgr/connect")
    .body(body)
    .header("Content-Type", "application/json")
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
               Err(String::from(format!("Could not create session from profile")))
            }
        },
        Err(_) => Err(String::from("Could not send request"))
    }
}