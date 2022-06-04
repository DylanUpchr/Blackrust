/** File
 * Author:	   Dylan Upchurch
 * Date:		   2021-05-13
 * Desc:		   BlackrustLib Session module
 */

use tokio::net::UdpSocket;
use crate::profile::Profile;
use async_trait::async_trait;
use erased_serde::{Serialize, serialize_trait_object};

#[async_trait]
 pub trait Session: Serialize {
    async fn connect(&mut self) -> Result<(), String>;
    async fn keepalive(&self) -> Result<bool, String>;
    fn disconnect(&mut self);
    fn id(&self) -> u32;
    fn rfb_port(&self) -> u16;
    fn name(&self) -> &str;
 }

 serialize_trait_object!(Session);

 pub trait UdpSession : Session {
   fn new(socket: UdpSocket, profile: Profile, display_number: u16) -> Self where Self: Sized;
 }