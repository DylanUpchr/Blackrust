/**
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust configuration manager crate (Profile/settings management)
 */
use blackrust_lib::*;

pub fn get_profiles(query: String) -> Result<Profiles, String> {
    println!("Received query: {}", query);
    let mut results = Profiles(vec![]);
    results.0.push(Profile::new());
    results.0.push(Profile::new());
    //return Err("Error".to_string());
    println!("Returned results: {}", results);
    return Ok(results);
}