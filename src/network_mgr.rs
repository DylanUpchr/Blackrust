/** File
 * Author:		Dylan Upchurch
 * Date:		2021-02-03
 * Desc:		Blackrust network manager crate
 */
use std::process::{Command, Output};
use std::str;

pub fn get_hostname() -> String {
    let output: Output = exec_nmcli_command(vec!("general", "hostname")).unwrap();
    str::from_utf8(&output.stdout).unwrap().to_string()
}
pub fn load_profiles()/* -> Vec<Vec<String>>*/{
    let output: Output = exec_nmcli_command(vec!("connection", "show")).unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let mut stdout_lines: Vec<&str> = stdout.split("\n").collect::<Vec<&str>>();
    stdout_lines.remove(0);
    /*stdout_lines.into_iter().map(|line| {
        line.split("")
    });*/
    println!("{:?}", stdout_lines);

}
pub fn exec_nmcli_command(args: Vec<&str>) -> Result<Output, std::io::Error> {
    Command::new("nmcli").args(args).output()
}
