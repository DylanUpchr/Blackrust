use std::time::Duration;
use crate::profile::PortProtocol;
use crate::profile::NetworkManagerProfileType;
//Constants
pub const DATA_PATH: &str = "blackrust/data";
pub const PROFILES_FILENAME: &str = "profiles.toml";
pub const PROFILE_NAME: &str = "Empty profile";
pub const PROTOCOL_NAME: &str = "Local";
pub const PROTOCOL_PORT: u16 = 0;
pub const PROTOCOL_PORT_PROTOCOL: PortProtocol = PortProtocol::None;
pub const NETWORK_MANAGER_PROFILE_TYPE: NetworkManagerProfileType = NetworkManagerProfileType::Ethernet;
pub const NEGOTIATION_TIMEOUT: Duration = Duration::from_millis(50);