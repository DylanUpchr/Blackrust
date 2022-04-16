/** File
 * Author:		Dylan Upchurch
 * Date:		2021-02-03
 * Desc:		Blackrust network manager crate
 */
use std::process::Command;
use std::str;
use regex::Regex;
use itertools::Itertools;
use blackrust_lib::profile::{NetworkManagerProfile,NetworkManagerProfileType,Interface};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait NetworkTool {
    fn exec_command<'a>(&self, args: Vec<&'a str>) -> Result<String, String>;
}

pub struct NetworkManager;
impl NetworkTool for NetworkManager {
    /** Function
     * Name:	exec_command
     * Purpose:	Execute NetworkManager command using nmcli
     * Args:	(Vec<&str>) Command-line arguments
     * Returns: (Result<String, String>) Command stdout or stderr
     */
    fn exec_command<'a>(&self, args: Vec<&'a str>) -> Result<String, String> {
        let command = Command::new("nmcli").args(args.clone()).output();
        match command {
            Ok(output) => {
                if output.stderr.is_empty() && !output.stdout.is_empty() {
                    Ok(str::from_utf8(&output.stdout).unwrap().to_string())
                } else if !output.stderr.is_empty() {
                    Err(str::from_utf8(&output.stderr).unwrap().to_string())
                } else {
                    Ok(format!("Unknown status: {}", output.status))
                }
            }
            Err(_) => Err(format!("Could not execute nmcli command with args: {:?}", args).to_string()),
        }
    }
}
impl NetworkManager {
    pub fn new() -> NetworkManager {
        NetworkManager{}
    }
}
//impl NetworkTool for NetworkManager {
/** Function
 * Name:	get_hostname
 * Purpose:	Get system hostname
 * Args:	None
 * Returns: (String) System hostname
 */
pub fn get_hostname(network_tool: &dyn NetworkTool) -> Result<String, String> {
    match network_tool.exec_command(vec!["general", "hostname"]) {
        Ok(stdout) => Ok(stdout.trim().to_string()),
        Err(stderr) => Err(stderr), //TODO localized error message instead of stderr
    }
}

/** Function
 * Name:	set_hostname
 * Purpose:	Set system hostname
 * Args:	(&str) hostname to set
 * Returns: Result<String, String> Changed hostname or stderr from command
 */
pub fn set_hostname(network_tool: &dyn NetworkTool, hostname: &str) -> Result<String, String> {
    match network_tool.exec_command(vec!["general", "hostname", hostname]) {
        Ok(stdout) => Ok(stdout),
        Err(stderr) => Err(stderr), //TODO localized error message instead of stderr
    }
}

/** Function
 * Name:	get_all_interfaces
 * Purpose:	Get all network interfaces
 * Args:	None
 * Returns: Result<Vec<Interface>, String> Interfaces or error string
 */
pub fn get_all_interfaces(network_tool: &dyn NetworkTool) -> Result<Vec<Interface>, String> {
    let mut result: Vec<Interface> = vec![];
    match network_tool.exec_command(vec![
        "--fields",
        "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
        "device",
        "show",
    ]) {
        Ok(stdout) => {
            let stdout_lines: Vec<&str> = stdout.split("\n").collect::<Vec<&str>>();
            let re = Regex::new("\\s{2,}").unwrap();
            let fields: Vec<Vec<&str>> = stdout_lines
                .into_iter()
                .filter(|&line| line != "")
                .map(|line| re.split(line).collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();

            for (name, hw_addr, interface_type) in fields.into_iter().tuples() {
                result.push(Interface::new3(
                    name[1].to_string(),
                    hw_addr[1].to_string(),
                    interface_type[1].to_string(),
                ));
            }
            Ok(result)
        }
        Err(message) => Err(message),
    }
}

/** Function
 * Name:	get_interface_by_name
 * Purpose:	Get a network interface by name
 * Args:	(String) Interface name query
 * Returns: Result<Interface, String> Interface or error string
 */
pub fn get_interface_by_name(
    network_tool: &dyn NetworkTool,
    name: String,
) -> Result<Interface, String> {
    let interfaces: Vec<Interface> = get_all_interfaces(network_tool)?;
    let interface_result: Option<&Interface> = interfaces
        .iter()
        .find(|interface: &_| interface.name == name);

    match interface_result {
        Some(interface) => Ok(interface.clone()),
        None => {
            if name == "--" {
                Ok(Interface::new())
            } else {
                Err(String::from("Could not find interface."))
            }
        }
    }
}

/** Function
 * Name:	load_profiles
 * Purpose:	Load saved profiles from NetworkManager
 * Args:	None
 * Returns: (Vec<NetworkManagerProfile>) NetworkManager profiles
 */
pub fn load_all_profiles(network_tool: &dyn NetworkTool) -> Result<Vec<NetworkManagerProfile>, String> {
    let mut profiles: Vec<NetworkManagerProfile> = vec![];
    match network_tool.exec_command(vec!["connection", "show"]) {
        Ok(stdout) => {
            let mut stdout_lines: Vec<&str> = stdout.split("\n").collect::<Vec<&str>>();
            //Match line data instead of removing first line blindly? only affects test but cleaner
            stdout_lines.remove(0);
            let re = Regex::new("\\s{2,}").unwrap();
            stdout_lines.into_iter().for_each(|mut line| {
                if line != "" {
                    line = line.trim();
                    let line_data = re.split(line).collect::<Vec<&str>>();
                    let profile = NetworkManagerProfile::new4(
                        line_data[0].to_string(),
                        line_data[1].to_string(),
                        NetworkManagerProfileType::from_str(&line_data[2].to_string()).unwrap(),
                        get_interface_by_name(network_tool, line_data[3].to_string()).unwrap(),
                    );
                    profiles.push(profile);
                }
            });
            Ok(profiles)
        }
        Err(message) => Err(message),
    }
}

/** Function
 * Name:    get_simple_profile_by_id
 * Purpose:	Gets profile that has matching id
 * Args:	(String) Id with which to filter profiles
 * Returns:	(Result<NetworkManagerProfile, String>) Simple NetworkManagerProfile object or error string
 */
pub fn get_simple_profile_by_id(
    network_tool: &dyn NetworkTool,
    id: String,
) -> Result<NetworkManagerProfile, String> {
    let profiles: Vec<NetworkManagerProfile> = load_all_profiles(network_tool)?;
    let profile_result: Option<&NetworkManagerProfile>;
    profile_result = profiles.iter().find(|profile: &_| profile.uuid == id);

    match profile_result {
        Some(profile) => (return Ok(profile.clone())),
        None => (return Err(String::from("Could not find profile"))),
    }
}

/** Function
 * Name:    get_detailed_profile_by_id
 * Purpose:	Gets profile that has matching id
 * Args:	(String) Id with which to filter profiles
 * Returns:	(Result<NetworkManagerProfile, String>) Detailed NetworkManagerProfile object or error string
 */
pub fn get_detailed_profile_by_id(
    network_tool: &dyn NetworkTool,
    id: String,
) -> Result<NetworkManagerProfile, String> {
    let profiles: Vec<NetworkManagerProfile> = load_all_profiles(network_tool)?;
    let profile_result: Option<&NetworkManagerProfile>;
    profile_result = profiles.iter().find(|profile: &_| profile.uuid == id);

    match profile_result {
        Some(profile) => (return Ok(profile.clone())),
        None => (return Err(String::from("Could not find profile"))),
    }
}

/** Function
 * Name:    create_profile
 * Purpose:	Creates a new NetworkManagerProfile
 * Args:	None
 * Returns:	(Result<String, String>) NetworkManagerProfile id or error string
 */
pub fn create_profile(
    network_tool: &dyn NetworkTool,
    profile_type: NetworkManagerProfileType,
) -> Result<String, String> {
    match network_tool.exec_command(vec!["connection", "add", "type", profile_type.to_str()])
    {
        Ok(stdout) => {
            let re = Regex::new("(?P<id>([\\w]+-){4}([\\w]+))").unwrap();
            let caps = re.captures(&stdout);
            match caps {
                Some(cap) => Ok(String::from(cap.name("id").unwrap().as_str())),
                None => Err(String::from("Could not create network connection profile.")),
            }
        }
        Err(message) => Err(message),
    }
}

/** Function
 * Name:    modify_profile
 * Purpose:	Modifies a NetworkManagerProfile
 * Args:	(NetworkManagerProfile) profile: Modified NetworkManagerProfile
 * Returns:	(Result<(), String>) Empty result or error string
 */
pub fn modify_profile(
    network_tool: &dyn NetworkTool,
    profile: NetworkManagerProfile,
) -> Result<(), String> {
    let result = network_tool.exec_command(vec![
        "connection",
        "modify",
        &profile.uuid,
        "connection.id",
        &profile.name,
        "connection.interface-name",
        &profile.interface.name,
    ]);

    match result {
        Ok(_) => Ok(()),
        Err(stderr) => Err(stderr),
    }
}
/** Function
 * Name:    delete_profile
 * Purpose:	Deletes a NetworkManagerProfile
 * Args:	(NetworkManagerProfile) profile: NetworkManagerProfile to delete
 * Returns:	(Result<(), String>) Empty result or error string
 */
pub fn delete_profile(
    network_tool: &dyn NetworkTool,
    profile: NetworkManagerProfile,
) -> Result<(), String> {
    let result = network_tool.exec_command(vec!["connection", "delete", &profile.uuid]);

    match result {
        Ok(_) => Ok(()),
        Err(stderr) => Err(stderr),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!("connection", "show"), true)]
    #[case(vec!("show"), false)]
    fn exec_command_test(#[case] input: Vec<&'static str>, #[case] expected: bool) {
        let net_tool = NetworkManager::new();
        match net_tool.exec_command(input){
            Ok(_) => assert!(expected),
            Err(message) => assert!(!expected, "{}", message)
        }
    }
    #[rstest]
    #[case(vec!("general", "hostname"), "host-name", true)]
    #[case(vec!("general", "hostname"), "Could not get hostname", false)]
    fn get_hostname_test(#[case] input: Vec<&'static str>, #[case] expected_value: String, #[case] expected_status: bool){
        let mut mock_net_tool = MockNetworkTool::new();
        let return_value = expected_value.clone();
        mock_net_tool.expect_exec_command()
        .withf(move |f| f == &input)
        .times(1)
        .returning(move |_| if expected_status {
                Ok(return_value.clone())
            } else {
                Err(return_value.clone())
            });
        match get_hostname(&mock_net_tool) {
            Ok(hostname) => assert_eq!(hostname, expected_value),
            Err(message) => assert_eq!(message, expected_value)
        }
    }
    #[rstest]
    #[case(vec!("general", "hostname", "host-name"), "host-name", "", true)]
    #[case(vec!("general", "hostname", "host-name"), "host-name", "Could not set hostname", false)]
    fn set_hostname_test(#[case] input: Vec<&'static str>, #[case] hostname: String, #[case] expected_value: String, #[case] expected_status: bool){
        let mut mock_net_tool = MockNetworkTool::new();
        let return_value = expected_value.clone();
        mock_net_tool.expect_exec_command()
        .withf(move |f| f == &input)
        .times(1)
        .returning(move |_| if expected_status {
                Ok(return_value.clone())
            } else {
                Err(return_value.clone())
            });
        match set_hostname(&mock_net_tool, &hostname) {
            Ok(output) => assert_eq!(output, expected_value),
            Err(message) => assert_eq!(message, expected_value)
        }
    }
    #[rstest]
    #[case(
        vec![
            "--fields",
            "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
            "device",
            "show",
        ], 
        vec![Interface::new3(
            "lo".to_string(),
            "00:00:00:00:00:00".to_string(),
            "loopback".to_string()
        )],
        true,
        ""
    )]
    #[case(
        vec![
            "--fields",
            "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
            "device",
            "show",
        ], 
        vec![Interface::new()],
        false,
        "Could not get interfaces"
    )]
    fn get_all_interfaces_test(#[case] input: Vec<&'static str>, #[case] expected_value: Vec<Interface>, #[case] expected_status: bool, #[case] expected_message: String){
        let mut mock_net_tool = MockNetworkTool::new();
        let return_interface = expected_value[0].clone();
        let return_message = expected_message.clone();
        mock_net_tool.expect_exec_command()
        .withf(move |f| f == &input)
        .times(1)
        .returning(move |_| if expected_status { 
                Ok(
                    format!(
                        "GENERAL.DEVICE:  {}\nGENERAL.HWADDR:  {}\nGENERAL.TYPE:  {}", 
                        return_interface.name,
                        return_interface.mac_addr,
                        return_interface.interface_type
                    ))
            } else {
                Err(return_message.clone())
            });
        match get_all_interfaces(&mock_net_tool) {
            Ok(interfaces) => assert_eq!(interfaces, expected_value),
            Err(message) => assert_eq!(message, expected_message)
        }
    }
    #[rstest]
    #[case(
        vec![
            "--fields",
            "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
            "device",
            "show",
        ], 
        "lo",
        Interface::new3(
            "lo".to_string(),
            "00:00:00:00:00:00".to_string(),
            "loopback".to_string()
        ),
        true,
        ""
    )]
    #[case(
        vec![
            "--fields",
            "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
            "device",
            "show",
        ], 
        "--",
        Interface::new(),
        true,
        ""
    )]
    #[case(
        vec![
            "--fields",
            "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
            "device",
            "show",
        ], 
        "",
        Interface::new(),
        false,
        "Could not get interfaces"
    )]
    fn get_interface_by_name_test(#[case] input: Vec<&'static str>, #[case] interface_name: String, #[case] expected_value: Interface, #[case] expected_status: bool, #[case] expected_message: String){
        let mut mock_net_tool = MockNetworkTool::new();
        let return_interface = expected_value.clone();
        let return_message = expected_message.clone();
        mock_net_tool.expect_exec_command()
        .withf(move |f| f == &input)
        .times(1)
        .returning(move |_| if expected_status { 
            Ok(
                format!(
                    "GENERAL.DEVICE:  {}\nGENERAL.HWADDR:  {}\nGENERAL.TYPE:  {}", 
                    return_interface.name,
                    return_interface.mac_addr,
                    return_interface.interface_type
                ))
        } else {
            Err(return_message.clone())
            });
        match get_interface_by_name(&mock_net_tool, interface_name) {
            Ok(interface) => assert_eq!(interface, expected_value),
            Err(message) => assert_eq!(message, expected_message)
        }
    }
    #[rstest]
    #[case(
        vec!["connection", "show"],
        vec![NetworkManagerProfile::new4(
            "profile".to_string(),
            "00000000-0000-0000-0000-000000000000".to_string(),
            NetworkManagerProfileType::Ethernet,
            Interface::new()
        )],
        true,
        ""
    )]
    #[case(
        vec!["connection", "show"],
        vec![NetworkManagerProfile::new()],
        false,
        "Could not load profiles"
    )]
    fn load_all_profiles_test(#[case] input: Vec<&'static str>, #[case] expected_value: Vec<NetworkManagerProfile>, #[case] expected_status: bool, #[case] expected_message: String){
        let mut mock_net_tool = MockNetworkTool::new();
        let return_profile = expected_value[0].clone();
        let return_message = expected_message.clone();
        mock_net_tool.expect_exec_command()
        .withf(|f| f == &vec![
            "--fields",
            "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
            "device",
            "show",
        ])
        .times(0..2)
        .returning(|_| Ok("".to_string()));
        mock_net_tool.expect_exec_command()
        .withf(move |f| f == &input)
        .times(1)
        .returning(move |_| if expected_status {
                Ok(format!("\n{}  {}  {}  --", return_profile.name, return_profile.uuid, return_profile.profile_type.to_str()))
            } else {
                Err(return_message.clone())
            });
        match load_all_profiles(&mock_net_tool) {
            Ok(profiles) => assert_eq!(profiles, expected_value),
            Err(message) => assert_eq!(message, expected_message)
        }
    }
    #[rstest]
    #[case(
        vec![
            "connection",
            "add",
            "type",
            "ethernet"
        ],
        "00000000-0000-0000-0000-000000000000",
        NetworkManagerProfileType::Ethernet,
        "".to_string(),
        true
    )]
    #[case(
        vec![
            "connection",
            "add",
            "type",
            "ethernet"
        ],
        "00000000-0000-0000-0000-000000000000",
        NetworkManagerProfileType::Ethernet,
        "Could not create network connection profile".to_string(),
        false
    )]
    fn create_profile_test(#[case] input: Vec<&'static str>, #[case] profile_id: String, #[case] profile_type: NetworkManagerProfileType, #[case] expected_message: String, #[case] expected_status: bool){
        let mut mock_net_tool = MockNetworkTool::new();
        let return_profile_id = profile_id.clone();
        let return_error_message = expected_message.clone();
        mock_net_tool.expect_exec_command()
        .withf(move |f| f == &input)
        .times(1)
        .returning(move |_| if expected_status { 
                Ok(return_profile_id.clone())
            } else {
                Err(return_error_message.clone())
            });
        match create_profile(&mock_net_tool, profile_type) {
            Ok(id) => assert_eq!(id, profile_id),
            Err(message) => assert_eq!(message, expected_message)
        }
    }
    #[rstest]
    #[case(
        vec![
            "--fields",
            "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
            "device",
            "show",
        ],
        NetworkManagerProfile::new4(
            "profile".to_string(),
            "00000000-0000-0000-0000-000000000000".to_string(),
            NetworkManagerProfileType::Ethernet,
            Interface::new()
        ),
        "",
        true
    )]
    #[case(
        vec![
            "--fields",
            "GENERAL.DEVICE,GENERAL.HWADDR,GENERAL.TYPE",
            "device",
            "show",
        ],
        NetworkManagerProfile::new(),
        "Could not find profile",
        false
    )]
    fn get_simple_profile_by_id_test(
        #[case] input: Vec<&'static str>, 
        #[case] expected_value: NetworkManagerProfile, 
        #[case] expected_message: String, 
        #[case] expected_status: bool
    ){
        let mut mock_net_tool = MockNetworkTool::new();
        let return_profile = expected_value.clone();
        let return_message = expected_message.clone();
        mock_net_tool.expect_exec_command()
        .withf(move |f| f == &input)
        .times(0..2)
        .returning(|_| Ok("".to_string()));
        mock_net_tool.expect_exec_command()
        .withf(|f| f == &vec!["connection", "show"])
        .times(1)
        .returning(move |_| if expected_status {
                Ok(
                    format!(
                        "\n{}  {}  {}  --", 
                        return_profile.name, 
                        return_profile.uuid, 
                        return_profile.profile_type.to_str()
                    ))
            } else {
                Err(return_message.clone())
            });
        match get_simple_profile_by_id(&mock_net_tool, expected_value.uuid.clone()){
            Ok(profile) => assert_eq!(profile, expected_value),
            Err(message) => assert_eq!(message, expected_message)
        }
    }
    #[test]
    fn get_detailed_profile_by_id_test(){

    }
    #[rstest]
    fn modify_profile_test(){

    }
    #[rstest]
    #[case(
        vec!["connection", "delete", "00000000-0000-0000-0000-000000000000"],
        NetworkManagerProfile::new4(
            "profile".to_string(),
            "00000000-0000-0000-0000-000000000000".to_string(),
            NetworkManagerProfileType::Ethernet,
            Interface::new()
        ),
        "",
        true
    )]
    #[case(
        vec!["connection", "delete", "00000000-0000-0000-0000-000000000000"],
        NetworkManagerProfile::new4(
            "profile".to_string(),
            "00000000-0000-0000-0000-000000000000".to_string(),
            NetworkManagerProfileType::Ethernet,
            Interface::new()
        ),
        "Could not delete profile",
        false
    )]
    fn delete_profile_test(#[case] input: Vec<&'static str>, #[case] profile: NetworkManagerProfile, #[case] expected_message: String, #[case] expected_status: bool){
        let mut mock_net_tool = MockNetworkTool::new();
        let return_message = expected_message.clone();
        mock_net_tool.expect_exec_command()
        .withf(move |f| f == &input)
        .times(1)
        .returning(move |_| if expected_status {
                Ok(return_message.to_string())
        } else {
                Err(return_message.to_string())
        });
        match delete_profile(&mock_net_tool, profile){
            Ok(_) => assert!(true),
            Err(message) => assert_eq!(message, expected_message)
        }
    }
}