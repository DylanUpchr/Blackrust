/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust remote sessions crate
 */
mod remote_protocols;
use remote_protocols::xdmcp::{self, XDMCPSession};
use blackrust_lib::{profile::Profile, session::{Session,UdpSession}};
use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::UdpSocket;

pub struct RemoteSessionManager {
    pub sessions: Vec<Box<dyn Session>>
}

impl RemoteSessionManager {
    pub fn new() -> RemoteSessionManager{
        RemoteSessionManager {
            sessions: vec![]
        }
    }
    #[tokio::main]
    pub async fn create_session(&mut self, profile: Profile) -> Result<String, String> {
        let src_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let dst_port = profile.connection_settings.protocol.port;
        match IpAddr::from_str(&profile.connection_settings.ip_fqdn) {
            Ok(dst_addr) => {
                match remote_protocols::open_udp_socket(src_addr, dst_addr, dst_port).await {
                    Ok(socket) => {
                        /*let session_handle = task::spawn(xdmcp::open_session(&socket, &profile.network_profiles));
                        match session_handle.await {
                            Ok(_) => ({
                                let session: XDMCPSession = XDMCPSession::new(socket, session_handle, profile);
                                self.sessions.push(Box::new(session));
                                Ok(session.id)
                            }),
                            Err(_) => (Err(String::from("Could not create session")))
                        }*/
                        let mut session: XDMCPSession = XDMCPSession::new(socket, profile);
                        match session.connect().await {
                            Ok(()) => {
                                let id = session.id().to_string();
                                self.sessions.push(Box::new(session));
                                Ok(id)
                            },
                            Err(message) => Err(message)
                        }
                    },
                    Err(err) => {
                        Err(err)
                    }
                }
            }
            Err(err) => (
                todo!()
            ) // Try resolve fqdn
        }
    }
    pub fn disconnect_session(&mut self, session_id: String){
        match self.get_session_by_id(&session_id){
            Some(session) => {
                session.disconnect();
                self.sessions.retain(|x| x.id() != session_id)
            },
            None => todo!(),
        }
    }
    pub fn is_session_alive(&self, session_id: String){
        todo!();
    }
    fn get_session_by_id(&self, session_id: &String) -> Option<&Box<dyn Session>>{
        self.sessions.iter().find(|session: &&Box<dyn Session> | session.id() == *session_id)
    }
}

/*#[tokio::main]
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
}*/