/** File
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust common library
 */ 

pub mod profile{
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
    * Name:	        DNS
    * Purpose:      DNS object
    * Properties:   (Vec<String>) name_servers: Vec of name server IP addresses
    *               (Vec<String>) search_domains: Vec of search domain names
    */
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DNS{
        pub name_servers: Vec<String>,
        pub search_domains: Vec<String>
    }

    /** Struct
    * Name:	        VPN
    * Purpose:      VPN object
    * Properties:   ...
    */
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct VPN{

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
    * Properties:   (String) interface: Name of interface to apply configuration to
    *               (String) ipv4: IPv4 address of local host
    *               (String) ipv6: IPv6 address of local host
    *               (String) hostname: Hostname of local host
    *               (String) gateway: IP address of local gateway
    *               (DNS) DNS configuration
    *               (VPN) VPN  configuration
    */
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct NetworkSettings {
        pub interface: String,
        pub ipv4: String,
        pub ipv6: String,
        pub hostname: String,
        pub gateway: String,
        pub dns: DNS,
        pub vpn: VPN

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
        pub network_settings: Vec<NetworkSettings>
    }

    /** Struct
    * Name:	        Profiles
    * Purpose:      Profile Vector wrapper
    * Properties:   (Vec<Profile>) profile_vec: profile vector
    */
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Profiles{pub profile_vec: Vec<Profile>}

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

    impl DNS {
        /** Function
         * Name:	new
         * Purpose:	Default constructor for DNS object
         * Args:	None
         * Returns:	DNS object
         */
        pub fn new() -> DNS {
            return DNS::new2(
                vec!(String::new()),
                vec!(String::new())
            )
        }
        /** Function
         * Name:	new
         * Purpose:	Default constructor for DNS object
         * Args:	(Vec<String>) name_servers: Vec of name server IP addresses
         *          (Vec<String>) search_domains: Vec of search domain names
         * Returns:	DNS object
         */
        pub fn new2(name_servers: Vec<String>, search_domains: Vec<String>) -> DNS {
            return DNS {
                name_servers: name_servers,
                search_domains: search_domains
            }
        }
    }

    impl VPN {
        /** Function
         * Name:	new
         * Purpose:	Default constructor for ConnectionSettings object
         * Args:	None
         * Returns:	ConnectionSettings object
         */
        pub fn new() -> VPN {
            return VPN{}
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

    impl NetworkSettings {
        /** Function
         * Name:	new
         * Purpose:	Default constructor for NetworkSettings object
         * Args:	None
         * Returns:	NetworkSettings object
         */
        pub fn new() -> NetworkSettings {
            return NetworkSettings::new7(
                String::new(), 
                String::new(), 
                String::new(), 
                String::new(),
                String::new(),
                DNS::new(),
                VPN::new()
            )
        }

        /** Function
        * Name:	    new7
        * Purpose:	Full constructor for NetworkSettings object
        * Args:	    (String) interface: Name of interface to apply configuration to
        *           (String) ipv4: IPv4 address of local host
        *           (String) ipv6: IPv6 address of local host
        *           (String) hostname: Hostname of local host
        *           (String) gateway: IP address of local gateway
        *           (DNS) DNS configuration
        *           (VPN) VPN  configuration
        * Returns:	NetworkSettings object
        */
        pub fn new7(interface: String, ipv4: String, ipv6: String, hostname: String, gateway: String, dns: DNS, vpn: VPN) -> NetworkSettings {
            return NetworkSettings{
                interface: interface,
                ipv4: ipv4,
                ipv6: ipv6,
                hostname: hostname,
                gateway: gateway,
                dns: dns,
                vpn: vpn
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
                vec!(NetworkSettings::new())
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
        pub fn new3(name: String, connection_settings: ConnectionSettings, network_settings: Vec<NetworkSettings>) -> Profile{
            return Profile{
                id: Uuid::new_v4().to_string(),
                name: name,
                connection_settings: connection_settings,
                network_settings: network_settings
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

    /** Function
     * Name:	read_file
     * Purpose:	Read contents from file
     * Args:	(&str) File path
     * Returns:	(String) File contents
     */
    pub fn read_file(path: &str) -> String{
        fs::read_to_string(path).expect("Issue with reading the requested file.")
    }

    /** Function
     * Name:	write_file
     * Purpose:	Write contents to file
     * Args:	(&str) File path
     *          (&str) Content to write to file
     * Returns:	Profile object
     */
    pub fn write_file(path: &str, content: &str){
        fs::write(path, content).expect("Issue with writing to the requested file.")
    }
}
pub mod defaults {
    use crate::profile::PortProtocol;
    //Constants
    pub const DATA_PATH: &str = "/etc/blackrust/data";
    pub const PROFILES_FILENAME: &str = "profiles.toml";
    pub const PROFILE_NAME: &str = "Empty profile";
    pub const PROTOCOL_NAME: &str = "Local";
    pub const PROTOCOL_PORT: u16 = 0;
    pub const PROTOCOL_PORT_PROTOCOL: PortProtocol = PortProtocol::None;
}
