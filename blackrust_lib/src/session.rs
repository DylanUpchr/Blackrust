/** File
 * Author:		Dylan Upchurch
 * Date:		2021-05-13
 * Desc:		BlackrustLib Session module
 */
#[macro_export]
macro_rules! session_default_return_type {
    () => {
        Box<dyn Session<Socket = UdpSocket, Handle = JoinHandle<()>, Profile = Profile>>
    };
}

 pub trait Session {
    type Socket;
    type Handle;
    type Profile;

    fn connect(&self);
    fn keepalive(&self);
    fn disconnect(&self);
 }