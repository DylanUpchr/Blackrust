/** File
 * Author:		Dylan Upchurch
 * Date:		2021-05-13
 * Desc:		BlackrustLib Profile module
 */
use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};
use crate::defaults;

 /** Enum
 * Name:    PortProtocol
 * Members: TCP: Transmission Control Protocol 
 *          UDP: User Datagram Protocol
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PortProtocol{
    TCP,
    UDP,
    None
}

 /** Enum
 * Name:    NetworkManagerProfileType
 * Members: Ethernet: An ethernet connection profile
 *          Wifi: A Wifi connection profile
 *          Wireguard: A Wireguard VPN connection profile
 */
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NetworkManagerProfileType{
    Ethernet,
    Wifi,
    Wireguard
}

 impl NetworkManagerProfileType{
    /** Function
     * Name:	from_str
     * Purpose:	Get enum member by string
     * Args:	(&str) enum member value (lowercase)
     * Returns:	(Result<NetworkManagerProfileType, Self::Err>) NetworkManagerProfileType or empty Err
     */
    pub fn from_str(input: &str) -> Result<NetworkManagerProfileType, String> {
        match input {
            "ethernet" => Ok(NetworkManagerProfileType::Ethernet),
            "wifi" => Ok(NetworkManagerProfileType::Wifi),
            "wireguard" => Ok(NetworkManagerProfileType::Wireguard),
            _      => Err(String::from("Could not find specified type")),
        }
    }

     /** Function
     * Name:	to_str
     * Purpose:	Get string from enum member
     * Args:	(&NetworkManagerProfileType) enum member
     * Returns:	(String) enum member value
     */
    pub fn to_str(self: &NetworkManagerProfileType) -> &str {
        match self {
            NetworkManagerProfileType::Ethernet => "ethernet",
            NetworkManagerProfileType::Wifi => "wifi",
            NetworkManagerProfileType::Wireguard => "wireguard"
        }
    }
}

 /** Struct
 * Name:	        Protocol
 * Purpose:      Protocol object
 * Properties:   (String) name: Name of protocol
 *               (u16) port: Host protocol port
 *               (PortProtocol) port_protocol: Remote port protocol (TCP/UDP/None)
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Protocol{
    pub name: String,
    pub port: u16,
    pub port_protocol: PortProtocol
}

 /** Struct
 * Name:	        Interface
 * Purpose:      Interface object
 * Properties:   (String) name: Name of Interface
 *               (String) mac_addr: Hardware MAC address
 */
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Interface{
    pub name: String,
    pub mac_addr: String,
    pub interface_type: String
}

impl Interface {
    /** Function
     * Name:	new
     * Purpose:	Default constructor for Interface object
     * Args:	None
     * Returns:	NetworkManagerProfile object
     */
    pub fn new() -> Interface{
        Interface::new3(
            String::new(), 
            String::new(),
            String::new()
        )
    }

     /** Function
     * Name:	new3
     * Purpose:	Full constructor for Interface object
     * Args:	(String) name: Interface name
     *          (String) mac_addr: Physical MAC address of interface
     *          (String) interface_type: Interface type
     * Returns:	Interface object
     */
    pub fn new3(name: String, mac_addr: String, interface_type: String) -> Interface{
        Interface {
            name: name,
            mac_addr: mac_addr,
            interface_type: interface_type
        }
    }
}

 /** Struct
 * Name:	        ConnectionSettings
 * Purpose:      Contains fields used for connection to remote host
 * Properties:   (String) ip_fqdn: IP address or FQDN of remote host
 *               (Protocol) protocol: Remote host protocol
 *               (String) extra_settings: Extra settings for remote session
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionSettings {
    pub ip_fqdn: String,
    pub protocol: Protocol,
    pub extra_settings: String
}

/** Struct
 * Name:	        NetworkSettings
 * Purpose:      Contains fields used for network configuration (LAN and VPN)
 * Properties:   (String) name: Name of the profile
 *               (String) uuid: Unique identifier of the profile
 *               (NetworkManagerProfileType) profile_type: Type of the profile (Ethernet, Wifi, etc.)
 *               (String) interface: Name of interface to apply configuration to
 **/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NetworkManagerProfile{
    pub name: String,
    pub uuid: String,
    pub profile_type: NetworkManagerProfileType,
    pub interface: Option<Interface>
}

 /** Struct
 * Name:	        Profile
 * Purpose:      Profile object
 * Properties:   (String) id: Profile UUIDv4
 *               (String) name: Profile name
 *               (ConnectionSettings) connection_settings: Remote connection configuration
 *               (Vec<NetworkSettings>) network_settings: Local networking interface configurations
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile{
    pub id: String,
    pub name: String,
    pub connection_settings: ConnectionSettings,
    pub network_profiles: Vec<NetworkManagerProfile>
}

 /** Struct
 * Name:	        Profiles
 * Purpose:      Profile Vector wrapper
 * Properties:   (Vec<Profile>) profile_vec: profile vector
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles{
    pub profile_vec: Vec<Profile>
}

 impl Protocol{
    /** Function
     * Name:	new
     * Purpose:	Default constructor for Protocol object
     * Args:	None
     * Returns:	Protocol object
     */
    pub fn new() -> Protocol {
        return Protocol::new3(
            String::from(defaults::PROTOCOL_NAME), 
            defaults::PROTOCOL_PORT, 
            defaults::PROTOCOL_PORT_PROTOCOL
        )
    }

     /** Function
     * Name:	    new3
     * Purpose:	Full constructor for Protocol object
     * Args:	    (String) name: Protocol name
     *           (u16) port: Remote host protocol port
     *               (PortProtocol) port_protocol: Remote port protocol (TCP/UDP/None)
     * Returns:	Profile object
     */
    pub fn new3(name: String, port: u16, port_protocol: PortProtocol) -> Protocol {
        return Protocol{
            name: name,
            port: port,
            port_protocol: port_protocol
        }
    }
}

 impl ConnectionSettings {
    /** Function
     * Name:	new
     * Purpose:	Default constructor for ConnectionSettings object
     * Args:	None
     * Returns:	ConnectionSettings object
     */
    pub fn new() -> ConnectionSettings {
        return ConnectionSettings::new3(
            String::new(), 
            Protocol::new(), 
            String::new()
        )
    }

     /** Function
     * Name:	    new3
     * Purpose:	Full constructor for ConnectionSettings object
     * Args:	    (String) ip_fqdn: IP address or FQDN of remote host
     *           (Protocol) protocol: Remote host protocol
     *           (String) extra_settings: Extra settings for remote session
     * Returns:	ConnectionSettings object
     */
    pub fn new3(ip_fqdn: String, protocol: Protocol, extra_settings: String) -> ConnectionSettings {
        return ConnectionSettings{
            ip_fqdn: ip_fqdn,
            protocol: protocol,
            extra_settings: extra_settings
        }
    }
}

 impl NetworkManagerProfile {
    /** Function
     * Name:	new
     * Purpose:	Default constructor for NetworkManagerProfile object
     * Args:	None
     * Returns:	NetworkManagerProfile object
     */
    pub fn new() -> NetworkManagerProfile {
        return NetworkManagerProfile::new4(
            String::new(), 
            Uuid::new_v4().to_string(),
            crate::defaults::NETWORK_MANAGER_PROFILE_TYPE,
            None
        )
    }

     /** Function
     * Name:	    new4
     * Purpose:	Full constructor for NetworkManagerProfile object
     * Args:	(String) name: Name of the profile
     *          (String) uuid: Unique identifier of the profile
     *          (NetworkManagerProfileType) profile_type: Type of the profile (Ethernet, Wifi, etc.)
     *          (Option<Interface>) interface: Interface that the profile uses
     * Returns:	NetworkManagerProfile object
     */
    pub fn new4(name: String, uuid: String, profile_type: NetworkManagerProfileType, interface: Option<Interface>) -> NetworkManagerProfile {
        return NetworkManagerProfile{
            name: name,
            uuid: uuid,
            profile_type: profile_type,
            interface: interface
        }
    }

 }

 impl Profile {

     /** Function
     * Name:	new
     * Purpose:	Default constructor for Profile object
     * Args:	None
     * Returns:	Profile object
     */
    pub fn new() -> Profile{
        return Profile::new3(
            String::from(defaults::PROFILE_NAME),
            ConnectionSettings::new(),
            vec!(NetworkManagerProfile::new())
        )
    }

     /** Function
     * Name:	    new3
     * Purpose:	Full constructor for Profile object
     * Args:	    (String) name: Profile name
     *           (ConnectionSettings) connection_settings: Remote connection configuration
     *           (Vec<NetworkSettings>) network_settings: Local networking interface configurations
     * Returns:	Profile object
     */
    pub fn new3(name: String, connection_settings: ConnectionSettings, network_profiles: Vec<NetworkManagerProfile>) -> Profile{
        return Profile{
            id: Uuid::new_v4().to_string(),
            name: name,
            connection_settings: connection_settings,
            network_profiles: network_profiles
        }
    }
}
impl Profiles {

     /** Function
     * Name:	new
     * Purpose:	Default constructor for Profiles object
     * Args:	None
     * Returns:	Profiles object
     */
    pub fn new() -> Profiles{
        return Profiles{
            profile_vec: vec![]
        }
    }

     /** Function
     * Name:	push
     * Purpose:	Clones profile_vec and returns new object with added profile
     * Args:	(&Profiles) Reference to Profiles object on which push was called
     * Returns:	Profiles object
     */
    pub fn push(&self, profile: Profile) -> Profiles{
        let mut newprofile_vec = self.profile_vec.to_vec();
        newprofile_vec.push(profile);
        return Profiles{
            profile_vec: newprofile_vec
        }
    }
}