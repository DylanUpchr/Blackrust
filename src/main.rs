#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use web_view::*;
//use xrandr::*;

fn main() {
	//let monitors = XHandle::open().unwrap().monitors().unwrap();

	//println!("{:?}", monitors);
	//let y = monitors[0].height_px;
	//let x = monitors[0].width_px * monitors.len() as i32;
	//println!("{}x{}", x, y);

	let html = combined_html_css_js();
	web_view::builder()
		.title("Fullscreen example")
		.content(Content::Html(html))
		.size(800, 600)
		.frameless(true)
		.debug(true)
		.user_data("")
		.invoke_handler(|_webview, arg| {
			use Cmd::*;
			match serde_json::from_str(arg).unwrap() {
				Init => (println!("init")),
				LoginAttempt { username, password } => (println!("{}:{}", username, password)),
			}
			Ok(())
		})
		.run()
		.unwrap();
}

fn combined_html_css_js() -> String {
	format!(
		"{}{}{}",
		include_str!("web/index.html"),
		inline_style(include_str!("web/style.css")),
		inline_script(include_str!("web/functions.js"))
	)
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
    LoginAttempt { username: String, password: String },
}
