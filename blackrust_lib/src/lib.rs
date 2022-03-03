/** File
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust common library
 */ 

pub mod profile{
    use uuid::Uuid;
    use serde_derive::{Serialize, Deserialize};
    use std::str::FromStr;
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
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum NetworkManagerProfileType{
        Ethernet,
        Wifi,
        Wireguard
    }

    impl FromStr for NetworkManagerProfileType{

        type Err = ();

        /** Function
         * Name:	from_str
         * Purpose:	Get enum member by string
         * Args:	(&str) enum member value (lowercase)
         * Returns:	(Result<NetworkManagerProfileType, Self::Err>) NetworkManagerProfileType or empty Err
         */
        fn from_str(input: &str) -> Result<NetworkManagerProfileType, Self::Err> {
            match input {
                "ethernet" => Ok(NetworkManagerProfileType::Ethernet),
                "wifi" => Ok(NetworkManagerProfileType::Wifi),
                "wireguard" => Ok(NetworkManagerProfileType::Wireguard),
                _      => Err(()),
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
        name: String,
        port: u16,
        port_protocol: PortProtocol
    }

    /** Struct
     * Name:	        Interface
     * Purpose:      Interface object
     * Properties:   (String) name: Name of Interface
     *               (String) mac_addr: Hardware MAC address
     */
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Interface{
        pub name: String,
        pub mac_addr: String
    }
    
    impl Interface {
        /** Function
         * Name:	new
         * Purpose:	Default constructor for Interface object
         * Args:	None
         * Returns:	NetworkManagerProfile object
         */
        pub fn new() -> Interface{
            Interface::new2(
                String::new(), 
                String::new()
            )
        }

        /** Function
         * Name:	new2
         * Purpose:	Full constructor for Interface object
         * Args:	(String) name: Interface name
         *          (String) mac_addr: Physical MAC address of interface
         * Returns:	Interface object
         */
        pub fn new2(name: String, mac_addr: String) -> Interface{
            Interface {
                name: name,
                mac_addr: mac_addr
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
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct NetworkManagerProfile{
        pub name: String,
        pub uuid: String,
        pub profile_type: NetworkManagerProfileType,
        pub interface: Interface
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
                Interface::new()
            )
        }

        /** Function
         * Name:	    new4
         * Purpose:	Full constructor for NetworkManagerProfile object
         * Args:	(String) name: Name of the profile
         *          (String) uuid: Unique identifier of the profile
         *          (NetworkManagerProfileType) profile_type: Type of the profile (Ethernet, Wifi, etc.)
         *          (String) interface: Name of interface to apply configuration to
         * Returns:	NetworkManagerProfile object
         */
        pub fn new4(name: String, uuid: String, profile_type: NetworkManagerProfileType, interface: Interface) -> NetworkManagerProfile {
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
}
pub mod file {
    use std::fs;
    use std::path::Path;

    /** Function
     * Name:	read_file
     * Purpose:	Read contents from file
     * Args:	(&str) File path
     * Returns:	(String) File contents
     */
    pub fn read_file(path: &Path) -> String{
        fs::read_to_string(path).expect("Issue with reading the requested file.")
    }

    /** Function
     * Name:	write_file
     * Purpose:	Write contents to file
     * Args:	(&str) File path
     *          (&str) Content to write to file
     * Returns:	Profile object
     */
    pub fn write_file(path: &Path, content: &str){
        fs::write(path, content).expect("Issue with writing to the requested file.")
    }
}
pub mod defaults {
    use crate::profile::PortProtocol;
    use crate::profile::NetworkManagerProfileType;
    //Constants
    pub const DATA_PATH: &str = "/etc/blackrust/data";
    pub const PROFILES_FILENAME: &str = "profiles.toml";
    pub const PROFILE_NAME: &str = "Empty profile";
    pub const PROTOCOL_NAME: &str = "Local";
    pub const PROTOCOL_PORT: u16 = 0;
    pub const PROTOCOL_PORT_PROTOCOL: PortProtocol = PortProtocol::None;
    pub const NETWORK_MANAGER_PROFILE_TYPE: NetworkManagerProfileType = NetworkManagerProfileType::Ethernet;
}
