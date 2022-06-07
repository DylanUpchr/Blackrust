use yew::html::ImplicitClone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PortProtocol{
    TCP,
    UDP,
    None
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Protocol{
    pub name: String,
    pub port: u16,
    pub port_protocol: PortProtocol
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NetworkManagerProfileType{
    Ethernet,
    Wifi,
    Wireguard
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Interface{
    pub name: String,
    pub mac_addr: String,
    pub interface_type: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConnectionSettings {
    pub ip_fqdn: String,
    pub protocol: Protocol,
    pub extra_settings: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NetworkManagerProfile{
    pub name: String,
    pub uuid: String,
    pub profile_type: NetworkManagerProfileType,
    pub interface: Option<Interface>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Profile{
    pub id: String,
    pub name: String,
    pub connection_settings: ConnectionSettings,
    pub network_profiles: Vec<NetworkManagerProfile>
}

impl ImplicitClone for Profile {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles{
    pub profile_vec: Vec<Profile>
}