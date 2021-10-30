/** File
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust common library
 */ 

pub mod Structs{
    use uuid::Uuid;
    use std::fmt;
    use serde_derive::{Serialize, Deserialize};

    /** Struct
    * Name:	        Protocol
    * Purpose:      Protocol object
    * Properties:   (String) name: Name of protocol
    */
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Protocol{
        name: String
    }

    /** Struct
    * Name:	        Profile
    * Purpose:      Profile object
    * Properties:   (String) id: Profile UUIDv4
    *               (String) name: Profile name
    *               (String) ip_fqdn: IP address or FQDN of remote host
    *               (Protocol) protocol: Remote protocol
    *               (String) conn_settings: Extra settings for connection
    */
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Profile{
        pub id: String,
        pub name: String,
        pub ip_fqdn: String,
        protocol: Protocol,
        conn_settings: String
    }

    /** Struct
    * Name:	        Profiles
    * Purpose:      Profile Vector wrapper
    * Properties:   (Vec<Profile>) profile_vec: profile vector
    */
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Profiles{pub profile_vec: Vec<Profile>}

    impl Profile{

        /** Function
         * Name:	new
         * Purpose:	Default constructor for Profile object
         * Args:	None
         * Returns:	Profile object
         */
        pub fn new() -> Profile{
            return Profile::new4(
                String::from("test"), 
                String::from("test"), 
                Protocol{name: String::from("test")},
                String::from("test"));
        }

        /** Function
        * Name:	    new4
        * Purpose:	Full constructor for Profile object
        * Args:	    (String) name: Profile name
        *           (String) ip_fqdn: Remote host IP address or FQDN
        *           (Protocol) protocol: Remote protocol
        *           (String) conn_settings: Extra setting for remote connection
        * Returns:	Profile object
        */
        pub fn new4(name: String, ip_fqdn: String, protocol: Protocol, conn_settings: String) -> Profile{
            return Profile{
                id: Uuid::new_v4().to_string(),
                name: name,
                ip_fqdn: ip_fqdn,
                protocol: protocol,
                conn_settings: conn_settings
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
    impl std::iter::FromIterator<Profile> for Profiles{
        fn from_iter<I: IntoIterator<Item=Profile>>(iter: I) -> Self {
            let profiles = Profiles::new();
    
            for i in iter {
                profiles.push(i);
            }
    
            profiles
        }
    }
    impl fmt::Display for Protocol {
        
        /** Function
         * Name:    fmt
         * Purpose:	JSON Display formatter
         * Args:	(&Protocol) Object being formatted
         *          (&mut fmt::Formatter) Formatter configuration 
         * Returns:	(Result) Formatted JSON string
         */
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, r#"{{"name":"{}"}}"#, self.name)
        }
    }
    impl fmt::Display for Profile {

        /** Function
         * Name:    fmt
         * Purpose:	JSON Display formatter
         * Args:	(&Profile) Object being formatted
         *          (&mut fmt::Formatter) Formatter configuration 
         * Returns:	(Result) Formatted JSON string
         */
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, r#"{{"id":"{}", "name":"{}", "ip_fqdn":"{}", "protocol":{}, "conn_settings":"{}"}}"#, 
            self.id, 
            self.name, 
            self.ip_fqdn, 
            self.protocol, 
            self.conn_settings)
        }
    }
    impl fmt::Display for Profiles {

        /** Function
         * Name:    fmt
         * Purpose:	JSON Display formatter
         * Args:	(&Profiles) Object being formatted
         *          (&mut fmt::Formatter) Formatter configuration 
         * Returns:	(Result) Formatted JSON string
         */
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.profile_vec.len() > 0{
                //Dynamically create JSON profile object array from Vec
                self.profile_vec.iter().enumerate().fold(Ok(()), |result, profile| {  //Fold executes a closure on all objects of a collection
                    result.and_then(|_| write!(f, r#"{}"{}": {}{}"#, //Write each object inside object array
                        (if profile.0 == 0 {"{"} else {""}), //Either opening curly bracket or empty string
                        profile.0, //Object index
                        profile.1, //Object JSON
                        (if self.profile_vec.len() - 1 == profile.0 {"}"} else {","}))) //Either comma seperating objects or closing curly bracket
                })
            } else {
                //Return empty JSON object if Vec contains no profiles
                write!(f, "{{}}")
            }
        }
    }
}
pub mod File{
    use std::fs;
    pub fn read_file(path: &str) -> String{
        fs::read_to_string(path).expect("Issue with reading the requested file.")
    }
    pub fn write_file(path: &str, toml: &str){
        fs::write(path, toml).expect("Issue with writing to the requested file.")
    }
}
