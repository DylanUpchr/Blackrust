/**
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust configuration manager crate (Profile/settings management)
 */
use blackrust_lib::*;

pub fn get_profiles(query: String) -> Result<Vec<Profile>, String> {
    println!("Received query: {}", query);
    let mut results = Vec::new();
    results.push(Profile::new());
    //return Err("Error".to_string());
    println!("Returned results: {:?}", results);
    return Ok(results);
}