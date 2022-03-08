/** File
 * Author:		Dylan Upchurch
 * Date:		2021-02-03
 * Desc:		Blackrust network manager crate
 */
use std::process::{Command, Output};
use std::str;
use regex::Regex;
use blackrust_lib::profile::{NetworkManagerProfile,NetworkManagerProfileType,Interface};

/** Function
 * Name:	get_hostname
 * Purpose:	Get system hostname
 * Args:	None
 * Returns: (String) System hostname
 */
pub fn get_hostname() -> String {
    exec_nmcli_command(vec!("general", "hostname")).unwrap()
}

pub fn get_all_interfaces() -> Result<Vec<Interface>, String> {
    let result = vec!();

    Ok(result)
}

pub fn get_interface_by_name(name: String) -> Result<Interface, String>{
    let interfaces: Vec<Interface> = get_all_interfaces()?;
    let interface_result: Option<&Interface> = interfaces.iter()
                        .find(|interface: &_| interface.name == name);

    match interface_result {
        Some(interface) => Ok(interface.clone()),
        None => Err(String::from("Could not find interface."))
    }
}

/** Function
 * Name:	set_hostname
 * Purpose:	Set system hostname
 * Args:	(&str) hostname to set
 * Returns: Result<String, String> Changed hostname or stderr from command
 */
pub fn set_hostname(hostname: &str) -> Result<String, String> {
    match exec_nmcli_command(vec!("general", "hostname", hostname)) {
        Ok(stdout) => Ok(stdout),
        Err(stderr) => Err(stderr) //TODO localized error message instead of stderr
    }
}

/** Function
 * Name:	load_profiles
 * Purpose:	Load saved profiles from NetworkManager
 * Args:	None
 * Returns: (Vec<NetworkManagerProfile>) NetworkManager profiles
 */
pub fn load_all_profiles() -> Result<Vec<NetworkManagerProfile>, String>{
    use std::str::FromStr;

    let mut profiles: Vec<NetworkManagerProfile> = vec!();
    let stdout = exec_nmcli_command(vec!("connection", "show")).unwrap();
    let mut stdout_lines: Vec<&str> = stdout.split("\n").collect::<Vec<&str>>();
    stdout_lines.remove(0);
    let re = Regex::new("\\s{2,}").unwrap();
    stdout_lines.into_iter().for_each(|line| {
        if line != "" {
            let line_data = re.split(line).collect::<Vec<&str>>();
            let interface = Interface::new2(line_data[3].to_string(), String::new());
            let profile = NetworkManagerProfile::new4(
                line_data[0].to_string(), 
                line_data[1].to_string(), 
                NetworkManagerProfileType::from_str(&line_data[2].to_string()).unwrap(), 
                interface
            );
            profiles.push(profile);
        }
    });
    Ok(profiles)
}

/** Function
 * Name:    get_simple_profile_by_id
 * Purpose:	Gets profile that has matching id
 * Args:	(String) Id with which to filter profiles
 * Returns:	(Result<NetworkManagerProfile, String>) Simple NetworkManagerProfile object or error string
 */
pub fn get_simple_profile_by_id(id: String) -> Result<NetworkManagerProfile, String>{
    let profiles: Vec<NetworkManagerProfile> = load_all_profiles()?;
    let profile_result: Option<&NetworkManagerProfile>;
    profile_result = profiles.iter()
                        .find(|profile: &_| profile.uuid == id);

    match profile_result {
        Some(profile) => (return Ok(profile.clone())),
        None => (return Err(String::from("Could not find profile.")))
    }
}

/** Function
 * Name:    get_detailed_profile_by_id
 * Purpose:	Gets profile that has matching id
 * Args:	(String) Id with which to filter profiles
 * Returns:	(Result<NetworkManagerProfile, String>) Detailed NetworkManagerProfile object or error string
 */
pub fn get_detailed_profile_by_id(id: String) -> Result<NetworkManagerProfile, String>{
    let profiles: Vec<NetworkManagerProfile> = load_all_profiles()?;
    let profile_result: Option<&NetworkManagerProfile>;
    profile_result = profiles.iter()
                        .find(|profile: &_| profile.uuid == id);

    match profile_result {
        Some(profile) => (return Ok(profile.clone())),
        None => (return Err(String::from("Could not find profile.")))
    }
}


/** Function
 * Name:    create_profile
 * Purpose:	Creates a new NetworkManagerProfile
 * Args:	None
 * Returns:	(Result<String, String>) NetworkManagerProfile id or error string
 */
pub fn create_profile() -> Result<String, String>{
    let new_id: String = String::new();
    Ok(new_id)
}

/** Function
 * Name:	exec_nmcli_command
 * Purpose:	Load saved profiles from NetworkManager
 * Args:	(Vec<&str>) Command-line arguments
 * Returns: (Result<String, String>) Command stdout or stderr
 */
pub fn exec_nmcli_command(args: Vec<&str>) -> Result<String, String> {
    let output: Output =  Command::new("nmcli").args(args).output().unwrap();
    
    if output.stdout.is_empty() {
        Err(str::from_utf8(&output.stderr).unwrap().to_string())
    } else {
        Ok(str::from_utf8(&output.stdout).unwrap().to_string())
    }

}
