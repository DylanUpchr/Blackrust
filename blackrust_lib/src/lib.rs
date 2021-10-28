/**
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust common library
 */ 
use uuid::Uuid;
#[derive(Debug)]
pub struct Protocol{
    name: String
}
#[derive(Debug)]
pub struct Profile{
    id: String,
    name: String,
    ip_fqdn: String,
    protocol: crate::Protocol,
    conn_settings: String
}
impl Profile{
    pub fn new() -> Profile{
        return Profile::new4(String::from("test"), String::from("test"), Protocol{name: String::from("test")},String::from("test"));
    }
    pub fn new4(name: String, ip_fqdn: String, protocol: Protocol, conn_settings: String) -> Profile{
        return Profile{
            id: Uuid::new_v4().to_string(),
            name: name,
            ip_fqdn: ip_fqdn,
            protocol: protocol,
            conn_settings: conn_settings
        }
    }
}
