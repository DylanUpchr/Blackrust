/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust XDMCP module
 */
use crate::network_mgr;
use crate::network_mgr::NetworkManager;
use async_trait::async_trait;
use blackrust_lib::{
    defaults,
    profile::{NetworkManagerProfile, Profile},
    session::{Session, UdpSession},
};
use std::net::IpAddr;
use std::process::Command;
use std::{env, process::Child};
use tokio::{
    net::UdpSocket,
    time
};use serde::ser::{Serialize, Serializer, SerializeStruct};

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
    Timeout,
    Unknown,
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
            _ => ProtocolOpCode::Unknown,
        }
    }
}

#[derive(PartialEq, Eq)]
enum ProtocolState {
    Query,
    CollectQuery,
    StartConnection,
    AwaitRequestResponse,
    AwaitManageResponse,
    StopConnection,
    AwaitAlive,
    RunSession,
}

enum ErrorState {
    Unwilling,
    Decline,
    Refuse,
    Failed,
    Timeout,
    Unknown,
    Xauth,
    OpenDisplay,
}

pub struct XDMCPSession {
    pub id: Option<u32>,
    pub name: String,
    pub socket: UdpSocket,
    pub profile: Profile,
    pub rfb_port: u16,
    xvnc_process: Option<Child>,
    display_number: u16
}

impl Serialize for XDMCPSession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("XDMCPSession", 3)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("rfb_port", &self.rfb_port)?;
        s.end()
    }
}

#[async_trait]
impl Session for XDMCPSession {
    async fn connect(&mut self) -> Result<(), String> {
        match self.open_session().await {
            Ok(rfb_port) => {
                self.rfb_port = rfb_port;
                Ok(())
            }
            Err(message) => Err(message),
        }
    }
    async fn keepalive(&self) -> Result<bool, String> {
        let mut data: Vec<u8> = vec![];
        match &self.id {
            Some(session_id) => {
                append_card_16(&mut data, self.display_number);
                append_card_32(&mut data, *session_id);
                send(&self.socket, ProtocolOpCode::KeepAlive, &data).await;
                match time::timeout(defaults::NEGOTIATION_TIMEOUT, recv(&self.socket)).await {
                    Ok(received_packet) => match received_packet {
                        Ok((received_op_code, _received_data)) => match received_op_code {
                            ProtocolOpCode::Alive => {
                                Ok(true)
                            },
                            _ => Err(String::from("Received out of sequence response from XDMCP manager"))
                        },
                        Err(_) => Ok(false)
                    },
                    Err(_) => Ok(false),
                }
            },
            None => Err(String::from("Session has no session id"))
        }
        
    }
    fn disconnect(&mut self) {
        match &mut self.xvnc_process {
            Some(child) => child.kill().unwrap(),
            None => (),
        }
    }
    fn id(&self) -> u32 {
        match &self.id {
            Some(id) => *id,
            None => 0_u32
        }
    }
    fn rfb_port(&self) -> u16 {
        self.rfb_port
    }
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}
impl UdpSession for XDMCPSession {
    fn new(socket: UdpSocket, profile: Profile, display_number: u16) -> XDMCPSession {
        XDMCPSession {
            id: None,
            name: profile.name.clone(),
            socket: socket,
            profile: profile,
            rfb_port: 0,
            xvnc_process: None,
            display_number: display_number
        }
    }
}

impl XDMCPSession {
    pub async fn open_session(&mut self) -> Result<u16, String> {
        let mut state: ProtocolState;
        let mut op: ProtocolOpCode = ProtocolOpCode::Query;
        let mut data: Vec<u8> = vec![0];
        let mut error_op_code: Option<ErrorState> = None;
        state = ProtocolState::CollectQuery;
        while state != ProtocolState::StopConnection && state != ProtocolState::RunSession {
            send(&self.socket, op, &data).await;
            match time::timeout(defaults::NEGOTIATION_TIMEOUT, recv(&self.socket)).await {
                Ok(received_result) => {
                    match received_result {
                        Ok((received_op_code, received_data)) => match state {
                            ProtocolState::CollectQuery => {
                                if received_op_code == ProtocolOpCode::Willing {
                                    op = ProtocolOpCode::Request;
                                    data = vec![];
                                    build_request_packet(
                                        &mut data,
                                        &self.profile.network_profiles,
                                        self.display_number,
                                    );
                                    state = ProtocolState::AwaitRequestResponse;
                                } else if received_op_code == ProtocolOpCode::Unwilling {
                                    state = ProtocolState::StopConnection;
                                    error_op_code = Some(ErrorState::Unwilling);
                                }
                            }
                            ProtocolState::StartConnection => {
                                op = ProtocolOpCode::Request;
                                data = vec![];
                                build_request_packet(
                                    &mut data,
                                    &self.profile.network_profiles,
                                    self.display_number,
                                );
                                state = ProtocolState::AwaitRequestResponse;
                            }
                            ProtocolState::AwaitRequestResponse => {
                                if received_op_code == ProtocolOpCode::Accept {
                                    op = ProtocolOpCode::Manage;
                                    data = vec![];
                                    self.id = Some(read_card_32(&received_data, 6));
                                    match build_manage_packet(
                                        &mut data,
                                        received_data,
                                        self.display_number,
                                    ) {
                                        Ok(_) => {
                                            {
                                                match super::open_display(self.display_number) {
                                                    Ok(child) => {
                                                        self.xvnc_process = Some(child);
                                                        state = ProtocolState::AwaitManageResponse
                                                    }
                                                    Err(_) => {
                                                        state = ProtocolState::StopConnection;
                                                        error_op_code =
                                                            Some(ErrorState::OpenDisplay);
                                                    }
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            state = ProtocolState::StopConnection;
                                            error_op_code = Some(ErrorState::Xauth);
                                        }
                                    }
                                } else if received_op_code == ProtocolOpCode::Decline {
                                    state = ProtocolState::StopConnection;
                                    error_op_code = Some(ErrorState::Decline);
                                }
                            }
                            ProtocolState::AwaitManageResponse => {
                                if received_op_code == ProtocolOpCode::Refuse {
                                    state = ProtocolState::StopConnection;
                                    error_op_code = Some(ErrorState::Refuse);
                                } else if received_op_code == ProtocolOpCode::Failed {
                                    state = ProtocolState::StopConnection;
                                    error_op_code = Some(ErrorState::Failed);
                                }
                            }
                            _ => (),
                        },
                        Err(message) => (println!("recv failed: {:?}", message)),
                    };
                }
                Err(_) => {
                    {
                        if state == ProtocolState::AwaitManageResponse {
                            state = ProtocolState::RunSession;
                        } else {
                            state = ProtocolState::StopConnection;
                            error_op_code = Some(ErrorState::Timeout);
                        }
                    }
                }
            }
        }
        match state {
            ProtocolState::StopConnection => match error_op_code {
                Some(op_code) => match op_code {
                    ErrorState::Unwilling => {
                        Err(String::from(
                            "X Server unwilling for XDMCP session negotiation",
                        ))
                    }
                    ErrorState::Decline => {
                        Err(String::from("X Server declined XDMCP session negotiation"))
                    }
                    ErrorState::Refuse => {
                        Err(String::from("X Server refused XDMCP session negotiation"))
                    }
                    ErrorState::Failed => (Err(String::from("XDMCP session negotiation failed"))),
                    ErrorState::Timeout => (Err(String::from("X Server timed out"))),
                    ErrorState::Unknown => {
                        Err(String::from("X Server sent unkown negotiation response"))
                    }
                    ErrorState::Xauth => {
                        Err(String::from("Could not add Xauthority authorization"))
                    }
                    ErrorState::OpenDisplay => (Err(String::from("Could not open display"))),
                },
                None => Ok(5900 + self.display_number),
            },
            ProtocolState::RunSession => Ok(5900 + self.display_number),
            _ => Err(String::from("Unknown negotiation error")),
        }
    }
}

pub async fn send(socket: &UdpSocket, op: ProtocolOpCode, data: &Vec<u8>) {
    let version: u16 = 1;
    let op_code: u16 = op as u16;

    let mut buf: Vec<u8> = vec![];
    append_card_16(&mut buf, version);
    append_card_16(&mut buf, op_code);
    append_array_8(&mut buf, data.to_vec());
    socket.send(&buf).await.unwrap();
}

pub async fn recv(socket: &UdpSocket) -> Result<(ProtocolOpCode, Vec<u8>), String> {
    let mut buf: Vec<u8> = vec![0u8; 512];
    time::sleep(time::Duration::from_millis(1)).await;
    match socket.recv(&mut buf).await {
        Ok(data_len) => {
            let opcode_bytes: [u8; 2] = buf[2..4].try_into().unwrap();
            Ok((
                ProtocolOpCode::from_u16(u16::from_be_bytes(opcode_bytes)),
                buf[..data_len].to_vec(),
            ))
        }
        Err(message) => Err(message.to_string()),
    }
}

fn build_request_packet(
    data: &mut Vec<u8>,
    network_profiles: &Vec<NetworkManagerProfile>,
    display_number: u16,
) {
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

fn build_manage_packet(
    data: &mut Vec<u8>,
    received_accept_packet: Vec<u8>,
    display_number: u16,
) -> Result<(), String> {
    let session_id: u32 = read_card_32(&received_accept_packet, 6);
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
    match add_xauth_cookie(&display_auth_name, display_auth_data, display_number).err() {
        Some(message) => (Err(message)),
        None => {
            append_card_32(data, session_id);
            append_card_16(data, display_number);
            append_array_8(data, display_class);
            Ok(())
        }
    }
}

fn add_xauth_cookie(
    auth_name: &str,
    auth_data: Vec<u8>,
    display_number: u16,
) -> Result<(), String> {
    let authfile_var = env::var("XAUTHORITY");
    let display_name: &str = &format!(":{}", display_number);
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
                        Ok(())
                    } else if !output.stderr.is_empty() {
                        Err(std::str::from_utf8(&output.stderr).unwrap().to_string())
                    } else {
                        Ok(())
                    }
                }
                Err(_) => Err(format!(
                    "Could not execute xauth command with args: {:?}. Stopping negotiation.",
                    args
                )
                .to_string()),
            }
        }
        Err(_) => {
            Err(format!("Could not find .Xauthority file. Stopping negotiation.").to_string())
        }
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
