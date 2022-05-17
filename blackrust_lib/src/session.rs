/** File
 * Author:	   Dylan Upchurch
 * Date:		   2021-05-13
 * Desc:		   BlackrustLib Session module
 */

use tokio::net::UdpSocket;
use crate::profile::Profile;
use async_trait::async_trait;

#[async_trait]
 pub trait Session {
    async fn connect(&mut self) -> Result<(), String>;
    fn keepalive(&self);
    fn disconnect(&mut self);
    fn id(&self) -> &str;
    fn rfb_port(&self) -> u16;
    fn name(&self) -> &str;
 }
 pub trait UdpSession : Session {
   fn new(socket: UdpSocket, profile: Profile) -> Self where Self: Sized;
 }