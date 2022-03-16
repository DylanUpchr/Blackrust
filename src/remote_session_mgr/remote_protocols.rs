/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust remote protocols module
 */
mod xdmcp;
use std::net::{SocketAddr, UdpSocket};

pub fn open_udp_socket(addr: SocketAddr) -> std::io::Result<UdpSocket>{
    let socket: UdpSocket = UdpSocket::bind(addr).unwrap();
    socket.connect(addr).unwrap();
    Ok(socket)
}