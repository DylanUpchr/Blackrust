/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust remote sessions crate
 */
mod remote_protocols;
use remote_protocols::xdmcp;
use blackrust_lib::{profile::Profile, session::Session, session_default_return_type};
use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use web_view::WebView;
use tokio::task::{self, JoinHandle};

pub struct RemoteSessionManager<'a> {
    pub sessions: Vec<session_default_return_type!()>,
    pub webview: Option<Arc<&'a mut WebView<'a, &'a str>>>
}

impl<'a> RemoteSessionManager<'a> {
    pub fn new() -> RemoteSessionManager<'a>{
        RemoteSessionManager {
            sessions: vec![],
            webview: None
        }
    }
    pub fn connect_session(&self, session: session_default_return_type!()){
        todo!();
    }
    pub fn disconnect_session(&self, session: session_default_return_type!()){
        todo!();
    }
    pub fn keepalive_session(&self, session: session_default_return_type!()){
        todo!();
    }
    pub fn attach_to_webview(&mut self, webview: Box<&'a mut WebView<'a, &'a str>>){
        self.webview = Some(Arc::new(*webview));
    }
}

#[tokio::main]
pub async fn connect(profile: Profile){
    let src_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let dst_port = profile.connection_settings.protocol.port;
    match IpAddr::from_str(&profile.connection_settings.ip_fqdn) {
        Ok(dst_addr) => {
            match remote_protocols::open_udp_socket(src_addr, dst_addr, dst_port).await {
                Ok(socket) => {
                    let session_handle = task::spawn(xdmcp::open_session(socket, profile.network_profiles));
                    match session_handle.await {
                        Ok(_) => (println!("Session handle ok")),
                        Err(_) => (println!("Session handle err"))
                    }
                },
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        _ => () // Try resolve fqdn
    }
}
pub fn show_error_message(message: String){
    println!("{}", message);
}