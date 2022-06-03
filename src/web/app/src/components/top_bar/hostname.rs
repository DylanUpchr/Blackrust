use yew::prelude::*;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

pub enum HostnameMsg {
    GetHostname,
    UpdateHostname { hostname: String }
}

pub struct Hostname {
    hostname: String
}

impl Component for Hostname {
    type Message = HostnameMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            hostname: String::from("hostname")
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HostnameMsg::GetHostname => {
                let future = spawn_local(async {
                    match get_hostname().await {
                        Ok(hostname) => ctx.link().clone().send_message(HostnameMsg::UpdateHostname { hostname: hostname }),
                        Err(_) => ()
                    } 
                });
                false
            },
            HostnameMsg::UpdateHostname { hostname } => {
                self.hostname = hostname;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{self.hostname.clone()}</div>
        }
    }
}

async fn get_hostname() -> Result<String, ()>{
    let call = Request::get("/path")
    .send()
    .await;

    match call {
        Ok(resp) => {
            if resp.ok() {
                Ok(resp.text().await.unwrap())
            } else {
                Err(())
            }
        },
        Err(_) => Err(())
    }
}