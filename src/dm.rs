/** File
 * Author:		Dylan Upchurch
 * Date:		2021-01-28
 * Desc:		Blackrust main crate (main entry point and opens webview)
 */
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate image_base64;
use web_view::*;
use regex::Regex;
use regex::Captures;
use blackrust_lib::profile::{Profile, NetworkManagerProfile};
mod config_mgr;
mod network_mgr;
mod remote_session_mgr;

/** Function
 * Name:	main
 * Purpose:	Main entry point for program
 * Args:	None
 * Returns:	None
 */
fn main() {
	match open_webview() {
		Ok(result) => {
			match result.run() {
				Ok(_) => (),
				_ => (println!("Could not run WebView"))
			}
		},
		Err(message) => (println!("{}", message))
	}
	println!("{:?}", network_mgr::exec_nmcli_command(vec!("show")));
}

/** Function
 * Name:	open_webview
 * Purpose:	Defines webview and opens it
 * Args:	None
 * Returns:	None
 */
fn open_webview() -> Result<WebView<'static, &'static str>, String> {
	let html = combined_html_css_js();
	let mut webview: WebView<'static, &'static str>;
	let webview_result = web_view::builder()
		.content(Content::Html(html))
		.size(1280, 720)
		.frameless(true)
		.debug(true)
		.user_data("")
		.invoke_handler(|webview, arg| {
			use Cmd::*;
			match serde_json::from_str::<Cmd>(arg) {
				Ok(cmd) => (
					match cmd {
						Init => (
							match network_mgr::get_hostname() {
								Ok(hostname) => webview.eval(&format!("setHostname({:?})", hostname)).unwrap(),
								Err(message) => (println!("{}", message))
							}),
						Debug { value } => (println!("{}", value)),
						Connect { profile } => ({
							println!("{:?}", profile);
							remote_session_mgr::connect(profile);
						}),
						QueryConnectionProfiles { callback, query } => ({
							match &config_mgr::get_profiles(query) {
								Ok(profiles) => webview.eval(
									&format!("{}({})", 
										callback,
										serde_json::to_string(
											profiles
										).unwrap()
									)
								)?,
								Err(message) => (println!("{}", message))
							};
						}),
						LoadConnectionProfile { callback, id } => ({
							match &config_mgr::get_profile_by_id(id) {
								Ok(profile) => webview.eval(
									&format!("{}({})",
										callback,
										serde_json::to_string(
											profile
										).unwrap()
									)
								)?,
								Err(message) => (println!("{}", message))
							}
						}),
						CreateConnectionProfile => ({
							let id = config_mgr::create_profile().unwrap();
							match &config_mgr::get_profiles("".to_string()) {
								Ok(profile) => webview.eval(
									&format!("loadQueriedConnectionProfilesSettings({})",
										serde_json::to_string(
											profile
										).unwrap()
									)
								)?,
								Err(message) => (println!("{}", message))
							}
							match &config_mgr::get_profile_by_id(id){
								Ok(profile) => (webview.eval(
									&format!("loadSelectedConnectionProfileSettings({})",
										serde_json::to_string(
											profile
										).unwrap()
									)
								)?),
								Err(message) => (println!("{}", message))
							}
						}),
						SaveConnectionProfile { profile } => (
							config_mgr::save_profile(profile)
						),
						DeleteConnectionProfile { profile } => (
							config_mgr::delete_profile(profile)
						),
						GetNetworkProfiles => (
							match &network_mgr::load_all_profiles() {
								Ok(profiles) => (webview.eval(
									&format!("loadNetworkProfiles({})",
										serde_json::to_string(
											profiles
										).unwrap()
									)
								)?),
								Err(message) => (println!("{}", message))
							}
						),
						LoadNetworkProfile { callback, id } => (
							match &network_mgr::get_simple_profile_by_id(id) {
								Ok(profile) => (webview.eval(
									&format!("{}({})",
										callback,
										serde_json::to_string(
											profile
										).unwrap()
									)
								)?),
								Err(message) => (println!("{}", message))
							}
						),
						CreateNetworkProfile => ({
							let id = network_mgr::create_profile().unwrap();
							match &network_mgr::load_all_profiles() {
								Ok(profiles) => (webview.eval(
									&format!("loadNetworkProfiles({})",
										serde_json::to_string(
											profiles
										).unwrap()
									)
								)?),
								Err(message) => (println!("{}", message))
							}
							match &network_mgr::get_detailed_profile_by_id(id) {
								Ok(profile) => (webview.eval(
									&format!("loadSelectedNetworkProfile({})",
										serde_json::to_string(
											profile
										).unwrap()
									)
								)?),
								Err(message) => (println!("{}", message))
							}
						}),
						SaveNetworkProfile { profile } => (
							match network_mgr::modify_profile(profile) {
								Err(message) => (println!("{}", message)),
								_ => ()
							}
						),
						DeleteNetworkProfile { profile } => (
							match network_mgr::delete_profile(profile) {
								Err(message) => (println!("{}", message)),
								_ => ()
							}
						),
						GetNetworkInterfaces => (
							match &network_mgr::get_all_interfaces() {
								Ok(interfaces) => (webview.eval(
									&format!("loadNetworkInterfaces({})",
										serde_json::to_string(
											interfaces
										).unwrap()
									)
								)?),
								Err(message) => (println!("{}", message))
							}
						)
					}
				),
				_ => (println!("Could not match command: {}", arg))
			}
			Ok(())
		})
		.build();
	
	match webview_result {
		Ok(webview) => Ok(webview),
		Err(_) => Err(String::from("Could not build webview"))
	}
}

/** Function
 * Name:	combined_html_css_js
 * Purpose:	Combines formatted HTML, CSS and JS all in one string
 * Args:	None
 * Returns:	(String) HTML webpage including CSS and JS
 */
fn combined_html_css_js() -> String {
	format!(
		"{}{}{}{}",
		base64_encode_images(include_str!("web/index.html"), "./src/web/"),
		inline_style(include_str!("web/style.css")),
		inline_script(include_str!("web/functions.js")),
		inline_script(include_str!("web/node_modules/@fortawesome/fontawesome-free/js/all.min.js"))
	)
}

/** Function
 * Name:	base64_encode_images
 * Purpose:	Encode all references to images in an HTML webpage to base64 by path
 * Args:	(&str) HTML webpage with image paths in src attributes
 * Returns:	(String) HTML webpage with base64 image strings in src attributes
 */
fn base64_encode_images(html: &str, web_dir_prefix: &str) -> String {
    let re = Regex::new(r"(\./.*\.png)").unwrap();
    let result = re.replace_all(html, |caps: &Captures| {
        format!("{}", image_base64::to_base64(&format!("{}{}", web_dir_prefix, &caps[0])))
    });
	return result.to_string();
}

/** Function
 * Name:	inline_style
 * Purpose:	Surround CSS styles in HTML tag for inclusion in webpage
 * Args:	(&str) CSS code
 * Returns: (String) CSS code surrounded in HTML tag
 */
fn inline_style(css: &str) -> String {
	format!(r#"<style type="text/css">{}</style>"#, css)
}

/** Function
 * Name:	inline_script
 * Purpose:	Surround JS styles in HTML tag for inclusion in webpage
 * Args:	(&str) JS code
 * Returns: (String) JS code surrounded in HTML tag
 */
fn inline_script(js: &str) -> String {
	format!(r#"<script type="text/javascript">{}</script>"#, js)
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
//Enum defining commands that JS can invoke
pub enum Cmd {
    Init,
	Debug { value: String },
	Connect { profile: Profile },
	QueryConnectionProfiles { callback: String, query: String },
	LoadConnectionProfile { callback: String, id: String },
	CreateConnectionProfile,
	SaveConnectionProfile { profile: Profile },
	DeleteConnectionProfile { profile: Profile },
	GetNetworkProfiles,
	LoadNetworkProfile { callback: String, id: String },
	CreateNetworkProfile,
	SaveNetworkProfile { profile: NetworkManagerProfile },
	DeleteNetworkProfile { profile: NetworkManagerProfile },
	GetNetworkInterfaces
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn open_webview_test(){
		match open_webview() {
			Ok(_) => assert!(true),
			Err(message) => assert!(false, "{}", message)
		}
	}
	#[test]
	fn base64_encode_images_test(){
		let test_img_path = "./img/base64_8x8_image.png";
		let test_img_path_prefix = "./test_resources/";
		let test_img_base64 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAgAAAAICAIAAABLbSncAAAACXBIWXMAAC4jAAAuIwF4pT92AAAAB3RJTUUH5gQMCyARQP6g9gAAABl0RVh0Q29tbWVudABDcmVhdGVkIHdpdGggR0lNUFeBDhcAAAB+SURBVAjXdY6xDoQgEAUf5BLK7eiFmGjEXzH6wXyFGKv1A+i2g2qvuPacbqoZo6oAeu8iAoCInHMAPgBqraUU5gdAjCGl5L1Hay3nvO3HME7DOG37kXNurVkRYX7O6wYA4Lxu5kdELF6wRBRjWJf55+syxxiIyKjq37h52/0CrmdF/bk0+fgAAAAASUVORK5CYII=";
		let html_unencoded = String::from(format!(r#"<img src="{}"></img>"#, test_img_path));
		let html_encoded = String::from(format!(r#"<img src="{}"></img>"#, test_img_base64));
		assert_eq!(html_encoded, base64_encode_images(&html_unencoded, test_img_path_prefix));
	}
}