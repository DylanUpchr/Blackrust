use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};
use log::{info, trace, warn};
use stylist::css;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::lib::{ Profile };
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
    DeselectProfile
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
                /*let link = ctx.link().clone();
                spawn_local(async move {
                    match connect().await {
                        Ok(hostname) => (),
                        Err(_) => ()
                    } 
                });*/
                match &self.selected_profile {
                    Some(profile) => {

                    },
                    None => ()
                }
                //self.event_bus.send(EventBusIOMsg::AddTab(0, "test".to_owned(), 0, AppRoute::Session { session_id: 0 }));
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

async fn connect() -> Result<(), ()> {
    let body = "";
    let call = Request::post("/rs_mgr/connect")
    .body(body)
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