/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust XDMCP module
 */
use std::net::UdpSocket;
use byteorder::{ByteOrder, NetworkEndian, BigEndian};

pub enum ProtocolOpCode {
    BroadcastQuery,
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
    pub fn to_u16(&self) -> u16{
        match &self {
            ProtocolOpCode::BroadcastQuery => 1,
            ProtocolOpCode::Query => 2,
            ProtocolOpCode::IndirectQuery => 3,
            ProtocolOpCode::ForwardQuery => 4,
            ProtocolOpCode::Willing => 5,
            ProtocolOpCode::Unwilling => 6,
            ProtocolOpCode::Request => 7,
            ProtocolOpCode::Accept => 8,
            ProtocolOpCode::Decline => 9,
            ProtocolOpCode::Manage => 10,
            ProtocolOpCode::Refuse => 11,
            ProtocolOpCode::Failed => 12,
            ProtocolOpCode::KeepAlive => 13,
            ProtocolOpCode::Alive => 14,
        }
    }
}

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

pub fn xdcmp_send(socket: UdpSocket, op: ProtocolOpCode, data: &[u8]){
    let version: u16 = 1;
    let op_code: u16 = ProtocolOpCode::Query.to_u16();
    let data_len_bytes: usize = data.len();
    let buf_len: usize = 2 + 2 + data_len_bytes;

    let buf: Vec<u8> = vec![0u8; buf_len];
    socket.send(&buf).unwrap();
}
pub fn xdcmp_recv(socket: UdpSocket){

}