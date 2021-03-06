use yew::html::ImplicitClone;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{ prelude::*, JsCast };

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PortProtocol {
    TCP,
    UDP,
    None
}

impl Default for PortProtocol {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Protocol {
    pub name: String,
    pub port: u16,
    pub port_protocol: PortProtocol
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NetworkManagerProfileType {
    Ethernet,
    Wifi,
    Wireguard
}

impl Default for NetworkManagerProfileType {
    fn default() -> Self {
        Self::Ethernet
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct Interface {
    pub name: String,
    pub mac_addr: String,
    pub interface_type: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ConnectionSettings {
    pub ip_fqdn: String,
    pub protocol: Protocol,
    pub extra_settings: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct NetworkManagerProfile {
    pub name: String,
    pub uuid: String,
    pub profile_type: NetworkManagerProfileType,
    pub interface: Option<Interface>
}

impl ImplicitClone for NetworkManagerProfile { }

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub connection_settings: ConnectionSettings,
    pub network_profiles: Vec<NetworkManagerProfile>
}

impl ImplicitClone for Profile { }

#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles {
    pub profile_vec: Vec<Profile>
}

impl Profiles {
    pub fn new() -> Profiles{
        return Profiles{
            profile_vec: vec![]
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: u32,
    pub name: String,
    pub rfb_port: u16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileFormData {
    pub profile: Profile
}

impl Into<JsValue> for ProfileFormData { 
    fn into(self) -> JsValue { 
        JsValue::from_str(
            &serde_json::to_string(
                &self
            ).unwrap()
        )
    }
}