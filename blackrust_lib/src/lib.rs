/**
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust common library
 */ 
use uuid::Uuid;
use std::fmt;
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
#[derive(Debug)]
pub struct Profiles(pub Vec<Profile>);

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
impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, r#"{{"name":"{}"}}"#, self.name)
    }
}
impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, r#"{{"id":"{}", "name":"{}", "ip_fqdn":"{}", "protocol":{}, "conn_settings":"{}"}}"#, self.id, self.name, self.ip_fqdn, self.protocol, self.conn_settings)
    }
}
impl fmt::Display for Profiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        self.0.iter().enumerate().fold(Ok(()), |result, profile| {
            result.and_then(|_| writeln!(f, r#"{}"{}": {}{}"#, 
                (if profile.0 == 0 {"{"} else {""}), 
                profile.0, 
                profile.1, 
                (if self.0.len() - 1 == profile.0 {"}"} else {","})))
        })
    }
}
