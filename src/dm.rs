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
mod config_mgr;
mod network_mgr;

/** Function
 * Name:	main
 * Purpose:	Main entry point for program
 * Args:	None
 * Returns:	None
 */
fn main() {
	let webview: WebView<'static, &'static str>;
	webview = open_webview();
	webview.run().unwrap();
}

/** Function
 * Name:	open_webview
 * Purpose:	Defines webview and opens it
 * Args:	None
 * Returns:	None
 */
fn open_webview() -> WebView<'static, &'static str> {
	let html = combined_html_css_js();
	let mut webview: WebView<'static, &'static str>;
	webview = web_view::builder()
		.content(Content::Html(html))
		.size(1280, 720)
		.frameless(true)
		.debug(true)
		.user_data("")
		.invoke_handler(|webview, arg| {
			use Cmd::*;
			match serde_json::from_str(arg).unwrap() {
				Init => (),
				Debug { value } => (println!("{}", value)),
				Connect { ip_fqdn, protocol, config} => (),
				QueryConnectionProfiles { callback, query } => (
					webview.eval(
						&format!("{}({})", 
							callback,
							serde_json::to_string(
								&config_mgr::get_profiles(query).unwrap()
							).unwrap()
						)
					)?
				),
				LoadConnectionProfile { callback, id } => (
					webview.eval(
						&format!("{}({})",
							callback,
							serde_json::to_string(
								&config_mgr::get_profile_by_id(id).unwrap()
							).unwrap()
						)
					)?
				),
				GetNetworkProfiles => (
					webview.eval(
						&format!("loadNetworkProfiles({})",
							serde_json::to_string(
								&network_mgr::load_all_profiles().unwrap()
							).unwrap()
						)
					)?
				),
				LoadNetworkProfile { id } => (
					webview.eval(
						&format!("loadSelectedNetworkProfile({})",
							serde_json::to_string(
								&network_mgr::get_profile_by_id(id).unwrap()
							).unwrap()
						)
					)?
				),
				CreateNetworkProfile => (
					webview.eval(
						&format!("loadSelectedNetworkProfile({})",
							serde_json::to_string(
								&network_mgr::get_profile_by_id(
									network_mgr::create_profile().unwrap()
								).unwrap()
							).unwrap()
						)
					)?
				)
			}
			Ok(())
		})
		.build()
		.unwrap();

	let hostname = network_mgr::get_hostname();
	webview.eval(&format!("setHostname({:?})", hostname)).unwrap();
	return webview;

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
		base64_encode_images(include_str!("web/index.html")),
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
fn base64_encode_images(html: &str) -> String {
	let web_dir_prefix = "./src/web/";
    let re = Regex::new(r"(\./img/.*\.png)").unwrap();
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
	Connect { ip_fqdn: String, protocol: String, config: String},
	QueryConnectionProfiles { callback: String, query: String },
	LoadConnectionProfile { callback: String, id: String },
	GetNetworkProfiles,
	LoadNetworkProfile { id: String },
	CreateNetworkProfile
}
