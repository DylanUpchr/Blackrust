/** File
 * Author:		Dylan Upchurch
 * Date:		2021-10-28
 * Desc:		Blackrust common library
 */ 

pub mod Structs{
    use uuid::Uuid;
    use std::fmt;
    
    /** Struct
    * Name:	        Protocol
    * Purpose:      Protocol object
    * Properties:   (String) name: Name of protocol
    */
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct Profile{
        id: String,
        name: String,
        ip_fqdn: String,
        protocol: Protocol,
        conn_settings: String
    }

    /** Struct
    * Name:	        Profiles
    * Purpose:      Profile Vector wrapper
    * Properties:   (Vec<Profile>) anonymous profile vector
    */
    #[derive(Debug)]
    pub struct Profiles(pub Vec<Profile>);

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
            //Dynamically create JSON profile object array from Vec
            self.0.iter().enumerate().fold(Ok(()), |result, profile| {  //Fold executes a closure on all objects of a collection
                result.and_then(|_| writeln!(f, r#"{}"{}": {}{}"#, //Write each object inside object array
                    (if profile.0 == 0 {"{"} else {""}), //Either opening curly bracket or empty string
                    profile.0, //Object index
                    profile.1, //Object JSON
                    (if self.0.len() - 1 == profile.0 {"}"} else {","}))) //Either comma seperating objects or closing curly bracket
            })
        }
    }
}
pub mod File{

}
