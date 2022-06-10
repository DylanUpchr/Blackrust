use yew::prelude::*;
use crate::lib::Session;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use std::process::Command;
use std::{env, process::Child};

pub struct SessionPage {
    pub session: Option<Session>
}

#[derive(Properties, PartialEq)]
pub struct SessionPageProperties {
    pub session_id: u32
}

pub enum SessionPageMsg {
    GetSession,
    UpdateSession { session: Session }
}

impl Component for SessionPage {
    type Message = SessionPageMsg;
    type Properties = SessionPageProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            session: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SessionPageMsg::GetSession => {
                let link = ctx.link().clone();
                let session_id = ctx.props().session_id;
                spawn_local(async move {
                    match get_session_by_id(session_id).await {
                        Ok(session) => link.send_message(SessionPageMsg::UpdateSession { session }),
                        Err(message) => log::info!("{}", message)
                    }
                });
                false  
            },
            SessionPageMsg::UpdateSession { session } => {
                self.session = Some(session);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let mut src = String::from("/noVNC/vnc.html?");

        match &self.session {
            Some(session) => {
                let port = (session.rfb_port + 64).to_string();
                log::info!("session: {:?}", session);
                log::info!("noVNC port: {}", port);
                let vnc_options = vec![
                    ("autoconnect", "true"),
                    ("host", "127.0.0.1"),
                    ("port", &port),
                    ("shared", "true"),
                    ("quality", "9"),
                    ("compression", "0"),
                    ("resize", "remote")
                ];
        
                vnc_options.iter().for_each(|option|
                    src = format!("{}{}={}&", src, option.0, option.1)
                );
            },
            None => ctx.link().send_message(SessionPageMsg::GetSession)
        }

        html! {
            <>
                <p>{format!("You are looking at Session {}", ctx.props().session_id)}</p>
                <iframe {src} width="1500px" height="800px"></iframe>
            </>
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