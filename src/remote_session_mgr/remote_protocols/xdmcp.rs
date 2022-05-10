use crate::network_mgr;
use crate::network_mgr::NetworkManager;
use blackrust_lib::profile::NetworkManagerProfile;
use std::env;
/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust XDMCP module
 */
use std::net::{IpAddr, UdpSocket};
use std::process::Command;
use xrandr::XHandle;
use tokio::{net, time};


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ProtocolOpCode {
    BroadcastQuery = 1,
    Query,
    IndirectQuery,
    ForwardQuery,
    Willing,
    Unwilling,
    Request,
    Accept,
    Decline,
    Manage,
    Refuse,
    Failed,
    KeepAlive,
    Alive,
}
impl ProtocolOpCode {
    pub fn from_u16(value: u16) -> ProtocolOpCode {
        match value {
            1 => ProtocolOpCode::BroadcastQuery,
            2 => ProtocolOpCode::Query,
            3 => ProtocolOpCode::IndirectQuery,
            4 => ProtocolOpCode::ForwardQuery,
            5 => ProtocolOpCode::Willing,
            6 => ProtocolOpCode::Unwilling,
            7 => ProtocolOpCode::Request,
            8 => ProtocolOpCode::Accept,
            9 => ProtocolOpCode::Decline,
            10 => ProtocolOpCode::Manage,
            11 => ProtocolOpCode::Refuse,
            12 => ProtocolOpCode::Failed,
            13 => ProtocolOpCode::KeepAlive,
            14 => ProtocolOpCode::Alive,
            _ => ProtocolOpCode::Query,
        }
    }
}

#[derive(PartialEq, Eq)]
enum ProtocolState {
    Query,
    CollectQuery,
    StartConnection,
    AwaitRequestResponse,
    Manage,
    AwaitManageResponse,
    StopConnection,
    AwaitAlive,
    RunSession,
}

pub fn send(socket: &UdpSocket, op: ProtocolOpCode, data: &Vec<u8>) {
    let version: u16 = 1;
    let op_code: u16 = op as u16;
    let data_len: u16 = data.len().try_into().unwrap();

    let mut buf: Vec<u8> = vec![];
    append_card_16(&mut buf, version);
    append_card_16(&mut buf, op_code);
    append_array_8(&mut buf, data.to_vec());
    socket.send(&buf).unwrap();
}

pub fn recv(socket: &UdpSocket) -> Result<(ProtocolOpCode, Vec<u8>), String> {
    let mut buf: Vec<u8> = vec![0u8; 512];
    match socket.recv(&mut buf) {
        Ok(data_len) => {
            let opcode_bytes: [u8; 2] = buf[2..4].try_into().unwrap();
            Ok((
                ProtocolOpCode::from_u16(u16::from_be_bytes(opcode_bytes)),
                buf[..data_len].to_vec(),
            ))
        }
        Err(message) => {
            println!("recv failed: {:?}", message);
            Err(message.to_string())
        }
    }
}

pub fn open_session(socket: &UdpSocket, network_profiles: Vec<NetworkManagerProfile>) {
    let mut state: ProtocolState = ProtocolState::Query;
    let mut op: ProtocolOpCode = ProtocolOpCode::Query;
    let mut data: Vec<u8> = vec![0];
    state = ProtocolState::CollectQuery;
    while state != ProtocolState::StopConnection && state != ProtocolState::RunSession {
        send(&socket, op, &data);
        match recv(&socket) {
            Ok((received_op_code, received_data)) => {
                println!("OpCode: {}", received_op_code as u16);
                match state {
                    ProtocolState::CollectQuery => {
                        if received_op_code == ProtocolOpCode::Willing {
                            op = ProtocolOpCode::Request;
                            data = vec![];
                            build_request_packet(&mut data, &network_profiles);
                            state = ProtocolState::AwaitRequestResponse;
                        } else if received_op_code == ProtocolOpCode::Unwilling {
                            state = ProtocolState::StopConnection;
                        }
                    }
                    ProtocolState::StartConnection => {
                        op = ProtocolOpCode::Request;
                        data = vec![];
                        build_request_packet(&mut data, &network_profiles);
                        state = ProtocolState::AwaitRequestResponse;
                    }
                    ProtocolState::AwaitRequestResponse => {
                        if received_op_code == ProtocolOpCode::Accept {
                            op = ProtocolOpCode::Manage;
                            data = vec![];
                            build_manage_packet(&mut data, received_data);
                            state = ProtocolState::AwaitManageResponse;
                        } else if received_op_code == ProtocolOpCode::Decline {
                            state = ProtocolState::StopConnection;
                        }
                    },
                    ProtocolState::AwaitManageResponse => {
                        if received_op_code == ProtocolOpCode::Refuse {
                            state = ProtocolState::StartConnection;
                        } else if received_op_code == ProtocolOpCode::Failed {
                            state = ProtocolState::StopConnection;
                        } else {
                            state = ProtocolState::RunSession;
                        }
                    },
                    _ => {},
                }
            }
            _ => state = ProtocolState::StopConnection,
        }
    }
}

fn build_request_packet(data: &mut Vec<u8>, network_profiles: &Vec<NetworkManagerProfile>) {
    let monitors = XHandle::open().unwrap().monitors().unwrap();

    let display_number = monitors.len() as u16;
    let mut conn_types: Vec<u16> = vec![];
    let mut interface_addrs: Vec<IpAddr> = vec![];
    let mut conn_addrs: Vec<Vec<u8>> = vec![];
    let network_tool = NetworkManager::new();
    network_profiles
        .iter()
        .for_each(|profile| match &profile.interface {
            Some(interface) => interface_addrs.append(
                &mut network_mgr::get_interface_addresses(&network_tool, interface.clone())
                    .unwrap(),
            ),
            None => (),
        });
    interface_addrs
        .iter()
        .for_each(|addr: &IpAddr| match *addr {
            IpAddr::V4(ip4) => {
                conn_addrs.push(ip4.octets().to_vec());
                conn_types.push(0);
            }
            IpAddr::V6(ip6) => {
                conn_addrs.push(ip6.octets().to_vec());
                conn_types.push(6);
            }
        });
    let auth_name: Vec<u8> = vec![];
    let auth_data: Vec<u8> = vec![];
    let auth_names: Vec<Vec<u8>> = vec![
        "MIT-MAGIC-COOKIE-1".as_bytes().to_vec(),
        "XDM-AUTHORIZATION-1".as_bytes().to_vec(),
        "SUN-DES-1".as_bytes().to_vec(),
    ];
    let mfr_display_id: Vec<u8> = vec![];

    append_card_16(data, display_number);
    append_array_16(data, conn_types);
    append_array_of_array_8(data, conn_addrs);
    append_array_8(data, auth_name);
    append_array_8(data, auth_data);
    append_array_of_array_8(data, auth_names);
    append_array_8(data, mfr_display_id);
}

fn build_manage_packet(data: &mut Vec<u8>, received_accept_packet: Vec<u8>) {
    let monitors = XHandle::open().unwrap().monitors().unwrap();

    let session_id: u32 = read_card_32(&received_accept_packet, 6);
    let display_number = monitors.len() as u16;
    let display_class: Vec<u8> = "MIT-unspecified".as_bytes().to_vec();
    let mut offset: usize = 10;
    let manager_auth_name = read_array_8(&received_accept_packet, offset);
    offset += manager_auth_name.len() + 2;
    let manager_auth_data = read_array_8(&received_accept_packet, offset);
    offset += manager_auth_data.len() + 2;
    let display_auth_name = read_array_8(&received_accept_packet, offset);
    offset += display_auth_name.len() + 2;
    let display_auth_name = String::from_utf8(display_auth_name).unwrap();
    let display_auth_data = read_array_8(&received_accept_packet, offset);
    add_xauth_cookie(&display_auth_name, display_auth_data, display_number);
    open_display(display_number);
    append_card_32(data, session_id);
    append_card_16(data, display_number);
    append_array_8(data, display_class);
}

fn add_xauth_cookie(auth_name: &str, auth_data: Vec<u8>, display_number: u16) {
    let authfile_var = env::var("XAUTHORITY");
    let display_name: &str = &format!("10.0.10.24:{}", display_number);
    let auth_data_str = &vec_u8_to_string(auth_data);
    match authfile_var {
        Ok(authfile_path) => {
            let args: Vec<&str> = vec![
                "-f",
                &authfile_path,
                "add",
                display_name,
                auth_name,
                auth_data_str,
            ];
            let command = Command::new("xauth").args(args.clone()).output();
            match command {
                Ok(output) => {
                    if output.stderr.is_empty() && !output.stdout.is_empty() {
                        //Ok(str::from_utf8(&output.stdout).unwrap().to_string())
                    } else if !output.stderr.is_empty() {
                        //Err(str::from_utf8(&output.stderr).unwrap().to_string())
                    } else {
                        //Ok(format!("Unknown status: {}", output.status))
                    }
                }
                Err(_) => (), //Err(format!("Could not execute nmcli command with args: {:?}", args).to_string()),
            }
        }
        Err(_) => (),
    }
}

fn open_display(display_number: u16) {
    let authfile_var = env::var("XAUTHORITY");
    let display_string = &format!(":{}", display_number);
    let vnc_port = &format!("{}", 5900 + display_number);
    match authfile_var {
        Ok(authfile_path) => {
            let xephyr_args = vec![
                "-listen",
                "tcp",
                display_string,
                "-auth",
                &authfile_path,
                "-screen",
                "800x600",
            ];
            let x11vnc_args = vec![
                "-localhost",
                "-display",
                display_string,
                "-auth",
                &authfile_path,
                "-rfbport",
                vnc_port,
            ];
            let xephyr_command = Command::new("Xephyr").args(xephyr_args).spawn();
            match xephyr_command {
                Ok(output) => {
                    let x11vnc_command = Command::new("x11vnc").args(x11vnc_args).spawn();
                    match x11vnc_command {
                        Ok(output) => (println!("{:?}", output)),
                        Err(err) => (println!("{}", err)),
                    }
                }
                Err(_) => (),
            }
        }
        Err(_) => (),
    }
}

fn read_card<const LENGTH: usize>(data: &Vec<u8>, offset: usize) -> [u8; LENGTH] {
    data[offset..offset + LENGTH].try_into().unwrap()
}

fn read_card_8(data: &Vec<u8>, offset: usize) -> u8 {
    u8::from_be_bytes(read_card::<1>(data, offset))
}

fn read_card_16(data: &Vec<u8>, offset: usize) -> u16 {
    u16::from_be_bytes(read_card::<2>(data, offset))
}

fn read_card_32(data: &Vec<u8>, offset: usize) -> u32 {
    u32::from_be_bytes(read_card::<4>(data, offset))
}

fn read_array_8(data: &Vec<u8>, offset: usize) -> Vec<u8> {
    let array_len = read_card_16(data, offset);
    let mut array_data: Vec<u8> = vec![];
    for card_offset in 0..array_len {
        array_data.push(read_card_8(data, offset + 2 + (card_offset as usize)));
    }
    array_data
}

fn append_card_8(data: &mut Vec<u8>, card_8: u8) {
    data.append(&mut card_8.to_be_bytes().to_vec());
}

fn append_card_16(data: &mut Vec<u8>, card_16: u16) {
    data.append(&mut card_16.to_be_bytes().to_vec());
}

fn append_card_32(data: &mut Vec<u8>, card_32: u32) {
    data.append(&mut card_32.to_be_bytes().to_vec());
}

fn append_array_8(data: &mut Vec<u8>, array_8: Vec<u8>) {
    append_card_16(data, array_8.len() as u16);
    data.append(&mut array_8.clone());
}

fn append_array_16(data: &mut Vec<u8>, array_16: Vec<u16>) {
    append_card_8(data, array_16.len() as u8);
    data.append(&mut vec_u16_to_be_vec_u8(array_16));
}

fn append_array_of_array_8(data: &mut Vec<u8>, array_of_array_8: Vec<Vec<u8>>) {
    append_card_8(data, array_of_array_8.len() as u8);
    array_of_array_8
        .iter()
        .for_each(|array_8: &Vec<u8>| append_array_8(data, array_8.clone()));
}

fn vec_u16_to_be_vec_u8(array_16: Vec<u16>) -> Vec<u8> {
    array_16
        .iter()
        .map(|card16: &u16| card16.to_be_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>()
        .concat()
}

fn vec_u8_to_string(data: Vec<u8>) -> String {
    let mut result = String::new();
    for i in 0..data.len() {
        result.push_str(&format!("{:02X}", data[i]))
    }
    result
}
