/** File
 * Author:		Dylan Upchurch
 * Date:		2022-05-13
 * Desc:		BlackrustLib File module
 */

use std::fs;
use std::path::Path; 
/** Function
 * Name:	read_file
 * Purpose:	Read contents from file
 * Args:	(&str) File path
 * Returns:	(String) File contents
 */
pub fn read_file(path: &Path) -> String{
    fs::read_to_string(path).expect("Issue with reading the requested file.")
} 
/** Function
 * Name:	write_file
 * Purpose:	Write contents to file
 * Args:	(&str) File path
 *          (&str) Content to write to file
 * Returns:	Profile object
 */
pub fn write_file(path: &Path, content: &str){
    fs::write(path, content).expect("Issue with writing to the requested file.")
} 
/** Function
 * Name:	create_path
 * Purpose:	Create path if not exists
 * Args:	(&str) Dir path
 * Returns:	None
 */
pub fn create_path(path: &Path){
    fs::create_dir_all(path).unwrap();
}