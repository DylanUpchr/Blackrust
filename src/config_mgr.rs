/** File
 * Author:		Dylan Upchurch
 * Date:		2022-01-27
 * Desc:		Blackrust configuration manager crate (Profile/settings management)
 */
use blackrust_lib::profile::*;
use blackrust_lib::file::*;
use blackrust_lib::defaults;
use std::path::Path;

/** Function
 * Name:    get_profiles
 * Purpose:	Gets all profiles responding to query
 * Args:	(String) Query with which to filter profiles
 * Returns:	(Result<Profiles, String>) Profiles object or error string
 */
pub fn get_profiles(query: String) -> Result<Profiles, String> {
    let mut profiles: Profiles = load_all_profiles()?;
    profiles.profile_vec = profiles.profile_vec.into_iter()
        .filter(|profile| {
            profile.name.to_lowercase().contains(&query.to_lowercase()) 
            || profile.connection_settings.ip_fqdn.to_lowercase().contains(&query.to_lowercase())
        })
        .collect();
    return Ok(profiles);
}

/** Function
 * Name:    get_profile_by_id
 * Purpose:	Gets profile that has matching id
 * Args:	(String) Id with which to filter profiles
 * Returns:	(Result<Profile, String>) Profile object or error string
 */
pub fn get_profile_by_id(id: String) -> Result<Profile, String> {
    let profiles: Profiles = load_all_profiles()?;
    let profile_result: Option<&Profile>;
    profile_result = profiles.profile_vec.iter()
                        .find(|profile: &_| profile.id == id);

    match profile_result {
        Some(profile) => (return Ok(profile.clone())),
        None => (return Err(String::from("Could not find profile.")))
    }
}

/** Function
 * Name:    load_all_profiles
 * Purpose:	Reads and returns all profiles from read config file
 * Args:	None
 * Returns:	(Result<Profiles, String>) Profiles object or error string
 */
pub fn load_all_profiles() -> Result<Profiles, String>{
    let path_str = format!("{}/{}", defaults::DATA_PATH, defaults::PROFILES_FILENAME);
    let path = Path::new(&path_str);
    let profiles: Profiles;
    if path.metadata().is_ok() {
        let toml = &read_file(path);
        profiles = toml::from_str(toml).unwrap()
    } else {
        profiles = Profiles::new();
    }
    return Ok(profiles);
}

/** Function
 * Name:    save_profiles
 * Purpose:	Saves Profiles object to default profiles file
 * Args:	(&Profiles) Profiles object
 * Returns:	None
 */
pub fn save_profiles(profiles: &Profiles){
    let toml = toml::Value::try_from(&profiles).unwrap();
    let path_str = format!("{}/{}", defaults::DATA_PATH, defaults::PROFILES_FILENAME);
    let path = Path::new(&path_str);
    write_file(&path, &format!("{}", toml));
}

/** Function
 * Name:    create_profile
 * Purpose:	Creates a new Profile
 * Args:	None
 * Returns:	(Result<String, String>) Profile id or error string
 */
pub fn create_profile() -> Result<String, String>{
    let profile: Profile = Profile::new();
    let mut profiles = load_all_profiles().unwrap();
    let id: String = profile.id.to_owned();
    profiles.profile_vec.push(profile);
    save_profiles(&profiles);
    Ok(id)
}