pub fn get_profiles(query: String) -> Result<Vec<String>, String> {
    println!("Received query: {}", query);
    let mut results = Vec::new();
    results.push(String::from("test"));
    //return Err("Error".to_string());
    println!("Returned results: {:?}", results);
    return Ok(results);
}