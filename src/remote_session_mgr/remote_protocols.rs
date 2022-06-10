/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust remote protocols module
 */
pub mod xdmcp;
use std::net::{SocketAddr, IpAddr};
use tokio::net::UdpSocket;
use std::process::Command;
use std::{env, process::Child};

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

fn open_display(display_number: u16) -> Result<Child, String> {
    let authfile_var = env::var("XAUTHORITY");
    let display_string: &str = &format!(":{}", display_number);
    match authfile_var {
        Ok(authfile_path) => {
                let xvnc_args = vec![
                    display_string,
                    "-listen",
                    "tcp",
                    "-auth",
                    &authfile_path,
                    "SecurityTypes=None",
                ];

                let vnc_port = 5900 + display_number;
                let ws_port = vnc_port + 64;

                let websockify_args = vec![
                    format!("0.0.0.0:{}", ws_port),
                    format!("localhost:{}", vnc_port),
                ];

                let websockify_command = Command::new("websockify").args(websockify_args).spawn();

                let xvnc_command = Command::new("Xvnc").args(xvnc_args).spawn();
                match xvnc_command {
                    Ok(child) => (Ok(child)),
                    Err(err) => (Err(err.to_string())),
                }
        },
        Err(_) => {
            Err(String::from(
                "Could not find ~/.Xauthority file",
            ))
        }
    }
}