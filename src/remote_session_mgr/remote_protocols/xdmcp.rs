/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust XDMCP module
 */
use std::net::UdpSocket;
//use byteorder::{ByteOrder, NetworkEndian, BigEndian};
use xrandr::XHandle;

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
    buf.append(&mut version.to_be_bytes().to_vec());
    buf.append(&mut op_code.to_be_bytes().to_vec());
    buf.append(&mut data_len.to_be_bytes().to_vec());
    buf.append(&mut data.clone());
    socket.send(&buf).unwrap();
}

pub fn recv(socket: &UdpSocket) -> Result<(ProtocolOpCode, Vec<u8>), String> {
    let mut buf: Vec<u8> = vec![0u8; 512];
    match socket.recv(&mut buf) {
        Ok(data_len) => {
            let opcode_bytes: [u8; 2] = buf[2..4].try_into().unwrap();
            Ok((ProtocolOpCode::from_u16(u16::from_be_bytes(opcode_bytes)), buf[5..data_len].to_vec()))
        }
        Err(message) => {
            Err(message.to_string())
        }
    }
}

pub fn open_session(socket: &UdpSocket) {
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
                        build_request_packet(&mut data);
                    },
                    _ => state = ProtocolState::StopConnection
                }
            },
            _ => state = ProtocolState::StopConnection
        }
    }
}

pub fn build_request_packet(data: &mut Vec<u8>){
    let outputs = XHandle::open().unwrap()
    .all_outputs().unwrap();

    let display_number = outputs.len() as u16;
    let conn_types: Vec<u16> = vec![0, 0, 6, 6];
    let conn_addrs: Vec<Vec<u8>> = vec![
        vec![192, 168, 0, 102],
        vec![10, 0, 10, 11],
        vec![42, 4, 238, 65, 0, 131, 48, 252, 13, 73u8, 115, 94, 102, 177, 37, 16],
        vec![254, 128, 0, 0, 0, 0, 0, 0, 171, 204, 231, 63, 57, 39, 98, 246]
    ];
    let auth_name: Vec<u8> = vec![];
    let auth_data: Vec<u8> = vec![];
    let auth_names: Vec<Vec<u8>> = vec![
        "MIT-MAGIC-COOKIE-1".as_bytes().to_vec(),
        "XDM-AUTHORIZATION-1".as_bytes().to_vec(),
        "SUN-DES-1".as_bytes().to_vec()
    ];
    let mfr_display_id: Vec<u8> = vec![];

    data.append(&mut display_number.to_be_bytes().to_vec());
    data.append(&mut (conn_types.len() as u8).to_be_bytes().to_vec());
    data.append(&mut vec_u16_to_be_vec_u8(conn_types));
    data.append(&mut (conn_addrs.len() as u8).to_be_bytes().to_vec());
    conn_addrs.iter().for_each(|x: &Vec<u8>| {
        data.append(&mut (x.len() as u16).to_be_bytes().to_vec());
        data.append(&mut x.clone());
    });
    data.append(&mut (auth_name.len() as u16).to_be_bytes().to_vec());
    data.append(&mut auth_name.clone());
    data.append(&mut (auth_data.len() as u16).to_be_bytes().to_vec());
    data.append(&mut auth_data.clone());
    data.append(&mut (auth_names.len() as u8).to_be_bytes().to_vec());
    auth_names.iter().for_each(|x: &Vec<u8>| {
        data.append(&mut (x.len() as u16).to_be_bytes().to_vec());
        data.append(&mut x.clone());
    });
    data.append(&mut (mfr_display_id.len() as u16).to_be_bytes().to_vec());
    data.append(&mut mfr_display_id.clone());
}

fn vec_u16_to_be_vec_u8(x: Vec<u16>) -> Vec<u8>{
    x.iter()
    .map(|y: &u16| {
        y.to_be_bytes().to_vec()
    })
    .collect::<Vec<Vec<u8>>>()
    .concat()
}