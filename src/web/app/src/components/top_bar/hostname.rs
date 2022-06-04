use yew::prelude::*;
use yew_feather::monitor::Monitor;
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

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(HostnameMsg::GetHostname);
        Self {
            hostname: String::from("hostname")
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HostnameMsg::GetHostname => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match get_hostname().await {
                        Ok(hostname) => link.send_message(HostnameMsg::UpdateHostname { hostname: hostname }),
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
            <div>
                <Monitor />
                {self.hostname.clone()}
            </div>
        }
    }
}

async fn get_hostname() -> Result<String, ()>{
    let call = Request::get("/net_mgr/hostname")
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