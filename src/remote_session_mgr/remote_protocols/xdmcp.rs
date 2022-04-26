/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust XDMCP module
 */
use std::net::{IpAddr, UdpSocket};
use xrandr::XHandle;
use crate::network_mgr;
use crate::network_mgr::{NetworkManager};
use blackrust_lib::profile::NetworkManagerProfile;

#[derive(Copy, Clone)]
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
    Alive
}
impl ProtocolOpCode {
    pub fn from_u16(value: u16) -> ProtocolOpCode{
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
            _ => ProtocolOpCode::Query
        }
    }
}

#[derive(PartialEq, Eq)]
enum ProtocolState {
    Start,
    Query,
    CollectQuery,
    BroadcastQuery,
    CollectBroadcastQuery,
    IndirectQuery,
    CollectIndirectQuery,
    StartConnection,
    AwaitRequestResponse,
    Manage,
    AwaitManageResponse,
    StopConnection,
    KeepAlive,
    AwaitAlive
}

pub fn send(socket: &UdpSocket, op: ProtocolOpCode, data: &Vec<u8>){
    let version: u16 = 1;
    let op_code: u16 = op as u16;
    let data_len: u16 = data.len().try_into().unwrap();

    let mut buf: Vec<u8> = vec!();
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
            Ok((ProtocolOpCode::from_u16(u16::from_be_bytes(opcode_bytes)), buf[..data_len].to_vec()))
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

    while state != ProtocolState::StopConnection {
        send(&socket, op, &data);
        match recv(&socket) {
            Ok((received_op_code, received_data)) => {
                println!("OpCode: {}", received_op_code as u16);
                match received_op_code {
                    ProtocolOpCode::Willing => {
                        op = ProtocolOpCode::Request;
                        state = ProtocolState::AwaitRequestResponse;
                        data = vec!();
                        build_request_packet(&mut data, &network_profiles);
                    },
                    ProtocolOpCode::Accept => {
                        op = ProtocolOpCode::Manage;
                        state = ProtocolState::AwaitManageResponse;
                        data = vec!();
                        build_manage_packet(&mut data, received_data);
                    }
                    _ => state = ProtocolState::StopConnection
                }
            },
            _ => state = ProtocolState::StopConnection
        }
    }
}

fn build_request_packet(data: &mut Vec<u8>, network_profiles: &Vec<NetworkManagerProfile>){
    let monitors = XHandle::open().unwrap()
    .monitors().unwrap();

    let display_number = monitors.len() as u16;
    let mut conn_types: Vec<u16> = vec![];
    let mut interface_addrs: Vec<IpAddr> = vec![];
    let mut conn_addrs: Vec<Vec<u8>> = vec![];
    let network_tool = NetworkManager::new();
    network_profiles.iter().for_each(|profile| {
        match &profile.interface {
            Some(interface) => interface_addrs.append(
                &mut network_mgr::get_interface_addresses(&network_tool, interface.clone()).unwrap()
            ),
            None => ()
        }
    });
    interface_addrs.iter().for_each(|addr: &IpAddr| {
        match *addr {
            IpAddr::V4(ip4) => {
                conn_addrs.push(ip4.octets().to_vec());
                conn_types.push(0);
            },
            IpAddr::V6(ip6) => {
                conn_addrs.push(ip6.octets().to_vec());
                conn_types.push(6);
            }
        }
    });
    let auth_name: Vec<u8> = vec![];
    let auth_data: Vec<u8> = vec![];
    let auth_names: Vec<Vec<u8>> = vec![
        "MIT-MAGIC-COOKIE-1".as_bytes().to_vec(),
        "XDM-AUTHORIZATION-1".as_bytes().to_vec(),
        "SUN-DES-1".as_bytes().to_vec()
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
    let monitors = XHandle::open().unwrap()
    .monitors().unwrap();

    let session_id: u32 = read_card_32(&received_accept_packet, 6);
    let display_number = monitors.len() as u16;
    let display_class: Vec<u8> = "MIT-unspecified".as_bytes().to_vec();

    append_card_32(data, session_id);
    append_card_16(data, display_number);
    append_array_8(data, display_class);
}

fn read_card<const LENGTH: usize>(data: &Vec<u8>, offset: usize,) -> [u8; LENGTH] {
    data[offset..offset+LENGTH].try_into().unwrap()
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

fn append_card_8(data: &mut Vec<u8>, card_8: u8){
    data.append(&mut card_8.to_be_bytes().to_vec());
}

fn append_card_16(data: &mut Vec<u8>, card_16: u16){
    data.append(&mut card_16.to_be_bytes().to_vec());
}

fn append_card_32(data: &mut Vec<u8>, card_32: u32){
    data.append(&mut card_32.to_be_bytes().to_vec());
}

fn append_array_16(data: &mut Vec<u8>, array_16: Vec<u16>){
    append_card_8(data, array_16.len() as u8);
    data.append(&mut vec_u16_to_be_vec_u8(array_16)); 
}

fn append_array_8(data: &mut Vec<u8>, array_8: Vec<u8>){
    append_card_16(data, array_8.len() as u16);
    data.append(&mut array_8.clone());
}

fn append_array_of_array_8(data: &mut Vec<u8>, array_of_array_8: Vec<Vec<u8>>){
    append_card_8(data, array_of_array_8.len() as u8);
    array_of_array_8.iter().for_each(|array_8: &Vec<u8>| append_array_8(data, array_8.clone()));
}

fn vec_u16_to_be_vec_u8(array_16: Vec<u16>) -> Vec<u8>{
    array_16.iter()
    .map(|card16: &u16| {
        card16.to_be_bytes().to_vec()
    })
    .collect::<Vec<Vec<u8>>>()
    .concat()
}