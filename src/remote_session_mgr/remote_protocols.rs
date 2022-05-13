/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust remote protocols module
 */
pub mod xdmcp;
use std::net::{SocketAddr, IpAddr};
use tokio::net::UdpSocket;

pub trait ProtocolTool {

}

pub async fn open_udp_socket(src_addr: IpAddr, dst_addr: IpAddr, dst_port: u16) -> Result<UdpSocket, String>{
    let src = SocketAddr::new(src_addr, 0);
    let dst = SocketAddr::new(dst_addr, dst_port);
    match UdpSocket::bind(src).await {
        Ok(socket) => {
            match socket.connect(dst).await {
                Ok(_) => Ok(socket),
                _ => Err(format!("Could not connect socket to {}:{}", dst.ip(), dst.port()))
            }
        },
        Err(_) => Err(format!("Could not bind UDP socket to {}:{}", src.ip(), src.port()))
    }
}