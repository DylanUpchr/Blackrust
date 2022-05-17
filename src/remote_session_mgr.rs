/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust remote sessions crate
 */
mod remote_protocols;
use remote_protocols::xdmcp::XDMCPSession;
use blackrust_lib::{profile::Profile, session::{Session,UdpSession}};
use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr};

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
                &session.disconnect();
                self.sessions.retain(|x| x.id() != session_id)
            },
            None => todo!(),
        }
    }
    pub fn is_session_alive(&self, session_id: String){
        todo!();
    }
    pub fn get_session_by_id(&mut self, session_id: &String) -> Option<&mut Box<dyn Session>>{
        self.sessions.iter_mut().find(|session: &&mut Box<dyn Session> | session.id() == *session_id)
    }
}