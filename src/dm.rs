#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate image_base64;
use web_view::*;
use regex::Regex;
use regex::Captures;
//use xrandr::*;

fn main() {
	open_webview();
}

fn open_webview() {
	let html = combined_html_css_js();
	let mut webview = web_view::builder()
		.content(Content::Html(html))
		.size(1260, 720)
		.frameless(true)
		.debug(true)
		.user_data("")
		.invoke_handler(|_webview, arg| {
			use Cmd::*;
			match serde_json::from_str(arg).unwrap() {
				Init => (println!("init")),
				Debug { value } => (println!("{}", value))
			}
			Ok(())
		})
		.build()
		.unwrap();

	let hostname = hostname::get().unwrap();
	webview.eval(&format!("setHostname({:?})", hostname)).unwrap();
	webview.run().unwrap();
}

fn combined_html_css_js() -> String {
	format!(
		"{}{}{}",
		base64_encode_images(include_str!("web/index.html")),
		inline_style(include_str!("web/style.css")),
		inline_script(include_str!("web/functions.js"))
	)
}

fn base64_encode_images(html: &str) -> String {
	let web_dir_prefix = "./src/web/";
    let re = Regex::new(r"(\./img/.*\.png)").unwrap();
    let result = re.replace_all(html, |caps: &Captures| {
		println!("{}", &caps[0]);
        format!("{}", image_base64::to_base64(&format!("{}{}", web_dir_prefix, &caps[0])))
    });
	return result.to_string();
}

fn inline_style(s: &str) -> String {
	format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
	format!(r#"<script type="text/javascript">{}</script>"#, s)
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
	Debug { value: String}
}
