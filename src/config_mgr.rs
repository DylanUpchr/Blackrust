/** File
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust configuration manager crate (Profile/settings management)
 */
use blackrust_lib::Structs::*;
use blackrust_lib::File;

/** Function
 * Name:    get_profiles
 * Purpose:	Gets all profiles responding to query
 * Args:	(String) Query with which to filter profiles
 * Returns:	(Result) Profiles object or error string
 */
pub fn get_profiles(query: String) -> Result<Profiles, String> {
    println!("Received query: {}", query);
    let mut results = load_all_profiles()?;
    results = results.push(Profile::new());
    results = results.push(Profile::new());
    temp_save_profiles(&results);
    //return Err("Error".to_string());
    println!("Returned results: {}", results);
    return Ok(results);
}

/** Function
 * Name:    load_all_profiles
 * Purpose:	Reads and returns all profiles from read config file
 * Args:	None
 * Returns:	(Result) Profiles object or error string
 */
pub fn load_all_profiles() -> Result<Profiles, String>{
    let toml = &File::read_file("/etc/blackrust/data/profiles.toml");
    let profiles: Profiles;
    if toml == "" {
        profiles = Profiles::new();
    } else {
        profiles = toml::from_str(toml).unwrap()
    }
    return Ok(profiles);
}
fn temp_save_profiles(profiles: &Profiles){
    let toml = toml::Value::try_from(&profiles).unwrap();
    File::write_file("/etc/blackrust/data/profiles.toml", &format!("{}", toml));
}