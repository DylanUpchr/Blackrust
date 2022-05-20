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

    fn get_next_display_number(&self) -> u16 {
        println!("nb sessions: {}", self.sessions.len());
        (self.sessions.len() as u16) + 1
    }

    #[tokio::main]
    pub async fn create_session(&mut self, profile: Profile) -> Result<u32, String> {
        let src_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let dst_port = profile.connection_settings.protocol.port;
        match IpAddr::from_str(&profile.connection_settings.ip_fqdn) {
            Ok(dst_addr) => {
                match remote_protocols::open_udp_socket(src_addr, dst_addr, dst_port).await {
                    Ok(socket) => {
                        let mut session: XDMCPSession = XDMCPSession::new(socket, profile, self.get_next_display_number());
                        match session.connect().await {
                            Ok(()) => {
                                let id = session.id();
                                self.sessions.push(Box::new(session));
                                println!("new nb sessions: {}", self.sessions.len());
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

    pub fn disconnect_session(&mut self, session_id: u32) -> Result<(), String>{
        match self.get_session_by_id(&session_id){
            Some(session) => {
                &session.disconnect();
                self.sessions.retain(|x| x.id() != session_id);
                Ok(())
            },
            None => Err(String::from("Could not find session")),
        }
    }

    pub fn get_session_by_id(&mut self, session_id: &u32) -> Option<&mut Box<dyn Session>>{
        self.sessions.iter_mut().find(|session: &&mut Box<dyn Session> | session.id() == *session_id)
    }
}