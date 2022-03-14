/** File
 * Author:		Dylan Upchurch
 * Date:		2021-03-13
 * Desc:		Blackrust XDMCP module
 */
use std::net::{SocketAddr, UdpSocket};
use byteorder::{ByteOrder, NetworkEndian, BigEndian};

enum ProtocolOpCode {
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
    pub fn to_u16(&self) -> Result<u16, String>{
        match &self {
            BroadcastQuery => Ok(1),
            Query => Ok(2),
            IndirectQuery => Ok(3),
            ForwardQuery => Ok(4),
            Willing => Ok(5),
            Unwilling => Ok(6),
            Request => Ok(7),
            Accept => Ok(8),
            Decline => Ok(9),
            Manage => Ok(10),
            Refuse => Ok(11),
            Failed => Ok(12),
            KeepAlive => Ok(13),
            Alive => Ok(14),
            _ => Err("Invalid OP code")
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
    let op_code: u16 = ProtocolOpCode::Query.to_u16().unwrap();
    let data_len_bytes: u16 = data.len();

    let buf: &[u8] = null;
    socket.send(buf);
}
pub fn xdcmp_recv(socket: UdpSocket){

}