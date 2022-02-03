/** File
 * Author:		Dylan Upchurch
 * Date:		2021-01-28
 * Desc:		Blackrust network manager crate
 */
use std::process::{Command, Output};
use std::str;

pub fn get_hostname() -> String {
    let output: Output = exec_nmcli_command(vec!("general", "hostname")).unwrap();
    str::from_utf8(&output.stdout).unwrap().to_string()
}
pub fn exec_nmcli_command(args: Vec<&str>) -> Result<Output, std::io::Error> {
    Command::new("nmcli").args(args).output()
}
