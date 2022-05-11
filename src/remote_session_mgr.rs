/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust remote sessions crate
 */
mod remote_protocols;
use remote_protocols::xdmcp;
use blackrust_lib::profile::*;
use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use tokio::task;

#[tokio::main]
pub async fn connect(profile: Profile){
    let src_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let dst_port = profile.connection_settings.protocol.port;
    match IpAddr::from_str(&profile.connection_settings.ip_fqdn) {
        Ok(dst_addr) => {
            let socket: UdpSocket = remote_protocols::open_udp_socket(src_addr, dst_addr, dst_port).unwrap();
            let future = async move {
                xdmcp::open_session(&socket, profile.network_profiles).await;
            };
            let session_handle = task::spawn(future);
            match session_handle.await {
                Ok(_) => (println!("Session handle ok")),
                Err(_) => (println!("Session handle err"))
            }
        }
        _ => () // Try resolve fqdn
    }
}