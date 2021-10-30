/** File
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust configuration manager crate (Profile/settings management)
 */
use blackrust_lib::Profile::*;
use blackrust_lib::File;

/** Function
 * Name:    get_profiles
 * Purpose:	Gets all profiles responding to query
 * Args:	(String) Query with which to filter profiles
 * Returns:	(Result) Profiles object or error string
 */
pub fn get_profiles(query: String) -> Result<Profiles, String> {
    println!("Received query: {}", query);
    let mut profiles: Profiles = load_all_profiles()?;
    profiles.profile_vec = profiles.profile_vec.into_iter()
        .filter(|profile| {profile.name.to_lowercase().contains(&query.to_lowercase()) || profile.ip_fqdn.to_lowercase().contains(&query.to_lowercase())})
        .collect();
    println!("Returned results: {}", profiles);
    return Ok(profiles);
}

/** Function
 * Name:    load_all_profiles
 * Purpose:	Reads and returns all profiles from read config file
 * Args:	None
 * Returns:	(Result) Profiles object or error string
 */
pub fn load_all_profiles() -> Result<Profiles, String>{
    let toml = &File::read_file(&format!("{}/{}", crate::DATA_PATH, crate::PROFILES_FILENAME));
    let profiles: Profiles;
    if toml == "" {
        profiles = Profiles::new();
    } else {
        profiles = toml::from_str(toml).unwrap()
    }
    return Ok(profiles);
}
pub fn save_profiles(profiles: &Profiles){
    let toml = toml::Value::try_from(&profiles).unwrap();
    File::write_file(&format!("{}/{}", crate::DATA_PATH, crate::PROFILES_FILENAME), &format!("{}", toml));
}