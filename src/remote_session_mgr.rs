/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust remote sessions crate
 */
mod remote_protocols;
use remote_protocols::xdmcp;
use blackrust_lib::profile::*;
//use async_net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};


pub fn connect(profile: Profile){
    
}
pub fn packet_test(){
    let src_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let dst_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 102));
    let dst_port: u16 = 177;
    let socket: UdpSocket = remote_protocols::open_udp_socket(src_addr, dst_addr, dst_port).unwrap();
    xdmcp::open_session(&socket);
}