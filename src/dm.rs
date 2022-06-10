/** File
 * Author:		Dylan Upchurch
 * Date:		2021-06-08
 * Desc:		Blackrust main crate (main entry point and opens webview)
 */
extern crate image_base64;
extern crate serde_json;
use serde::Deserialize;
use std::sync::Mutex;
use actix_files::Files;
use actix_web::{get, put, patch, post, delete, web, web::Data, App, HttpResponse, HttpServer};
mod config_mgr;
mod network_mgr;
mod remote_session_mgr;
use blackrust_lib::profile::{NetworkManagerProfile, Profile, NetworkManagerProfileType, ConnectionSettings};
use network_mgr::NetworkManager;
use remote_session_mgr::RemoteSessionManager;
use web_view::*;

macro_rules! result_http_response {
    ($e: expr)=>{
        match $e {
            Ok(value) => HttpResponse::Ok().body(serde_json::to_string(&value).unwrap()),
            Err(message) => HttpResponse::InternalServerError().body(message)
        }
    };
}

macro_rules! option_http_response {
    ($e: expr) => {
        match $e {
            Some(value) => HttpResponse::Ok().body(serde_json::to_string(&value).unwrap()),
            None => HttpResponse::NotFound().body("Resource not found")
        }
    };
}

struct AppState {
    remote_session_mgr: RemoteSessionManager,
    network_tool: NetworkManager
}

#[derive(Deserialize)]
struct ProfileFormData {
    pub profile: Profile
}

#[derive(Deserialize)]
struct HostnameFormData {
    hostname: String
}

#[derive(Deserialize)]
struct NetworkManagerProfileFormData {
    profile: NetworkManagerProfile
}

#[derive(Deserialize)]
struct NetworkManagerProfileTypeFormData {
    profile_type: NetworkManagerProfileType
}

/** Function
 * Name:	main
 * Purpose:	Main entry point for program
 * Args:	None
 * Returns:	None
 */
fn main() {
    start_actix(String::from("0.0.0.0:8080")).unwrap();
}

#[get("/hostname")]
async fn get_hostname(state: Data<Mutex<AppState>>) -> HttpResponse {
    let current_state = state.lock().unwrap();
    result_http_response!(network_mgr::get_hostname(&current_state.network_tool))
}

#[put("/hostname")]
async fn set_hostname(state: Data<Mutex<AppState>>, data: web::Form<HostnameFormData>) -> HttpResponse {
    let current_state = state.lock().unwrap();
    result_http_response!(network_mgr::set_hostname(&current_state.network_tool, &data.hostname))
}

#[get("/interfaces")]
async fn get_net_interfaces(state: Data<Mutex<AppState>>) -> HttpResponse {
    let current_state = state.lock().unwrap();
    result_http_response!(network_mgr::get_all_interfaces(&current_state.network_tool))
}

#[get("/profiles")]
async fn get_net_profiles(state: Data<Mutex<AppState>>) -> HttpResponse {
    let current_state = state.lock().unwrap();
    result_http_response!(network_mgr::load_all_profiles(&current_state.network_tool))
}

#[get("/profile/{id}")]
async fn get_net_profile(state: Data<Mutex<AppState>>, id: web::Path<String>) -> HttpResponse {
    let current_state = state.lock().unwrap();
    result_http_response!(network_mgr::get_simple_profile_by_id(&current_state.network_tool, id.to_string()))
}

#[post("/profile")]
async fn create_net_profile(state: Data<Mutex<AppState>>, data: web::Form<NetworkManagerProfileTypeFormData>) -> HttpResponse {
    let current_state = state.lock().unwrap();
    result_http_response!(network_mgr::create_profile(&current_state.network_tool, data.profile_type.clone()))
}

#[patch("/profile")]
async fn update_net_profile(state: Data<Mutex<AppState>>, data: web::Form<NetworkManagerProfileFormData>) -> HttpResponse {
    let current_state = state.lock().unwrap();
    result_http_response!(network_mgr::modify_profile(&current_state.network_tool, data.profile.clone()))
}

#[delete("/profile")]
async fn delete_net_profile(state: Data<Mutex<AppState>>, data: web::Form<NetworkManagerProfileFormData>) -> HttpResponse {
    let current_state = state.lock().unwrap();
    result_http_response!(network_mgr::delete_profile(&current_state.network_tool, data.profile.clone()))
}

#[get("/profiles")]
async fn get_conn_profiles() -> HttpResponse {
    result_http_response!(config_mgr::load_all_profiles())
}

#[get("/profiles/{query}")]
async fn query_conn_profiles(query: web::Path<String>) -> HttpResponse {
    result_http_response!(config_mgr::get_profiles(query.to_string()))
}

#[get("/profile/{id}")]
async fn get_conn_profile(id: web::Path<String>) -> HttpResponse {
    option_http_response!(config_mgr::get_profile_by_id(id.to_string()))
}

#[post("/profile")]
async fn create_conn_profile() -> HttpResponse {
    result_http_response!(config_mgr::create_profile())
}

#[patch("/profile")]
async fn update_conn_profile(data: web::Form<ProfileFormData>) -> HttpResponse {
    result_http_response!(config_mgr::save_profile(data.profile.clone()))
}

#[delete("/profile/{id}")]
async fn delete_conn_profile(id: web::Path<String>) -> HttpResponse {
    result_http_response!(config_mgr::delete_profile(id.to_string()))
}


#[get("/session/{id}")]
async fn get_session(state: Data<Mutex<AppState>>, id: web::Path<u32>) -> HttpResponse {
    let mut current_state = state.lock().unwrap();
    let result = &current_state.remote_session_mgr.get_session_by_id(id.into_inner());
    option_http_response!(result)
}
#[post("/connect")]
async fn connect(state: Data<Mutex<AppState>>, data: web::Json<ProfileFormData>) -> HttpResponse {
    let mut current_state = state.lock().unwrap();
    result_http_response!(&current_state.remote_session_mgr.create_session(data.profile.clone()))
}

#[post("/disconnect/{id}")]
async fn disconnect(state: Data<Mutex<AppState>>, id: web::Path<u32>) -> HttpResponse {
    let mut current_state = state.lock().unwrap();
    result_http_response!(&current_state.remote_session_mgr.disconnect_session(id.into_inner()))
}

#[actix_web::main]
async fn start_actix(bind_addr: String) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Mutex::new(AppState {
                remote_session_mgr: RemoteSessionManager::new(),
                network_tool: NetworkManager::new()
            })))
            .service(
                web::scope("/net_mgr")
                    .service(get_hostname)
                    .service(set_hostname)
                    .service(create_net_profile)
                    .service(update_net_profile)
                    .service(delete_net_profile)
                    .service(get_net_interfaces)
                    .service(get_net_profiles)
                    .service(get_net_profile)
            )
            .service(
                web::scope("/cfg_mgr")
                    .service(get_conn_profiles)
                    .service(query_conn_profiles)
                    .service(get_conn_profile)
                    .service(create_conn_profile)
                    .service(update_conn_profile)
                    .service(delete_conn_profile)
            )
            .service(
                web::scope("/rs_mgr")
                    .service(get_session)
                    .service(connect)
                    .service(disconnect)
            )
            .service(
                web::scope("/i18n")
            )
            .service(Files::new("/", "./src/web/app/dist/").index_file("index.html"))
            /*.default_service(
                web::route().to(|| HttpResponse::Found().header("Location", "/main").finish()),
            )*/
    })
    .bind(bind_addr).unwrap()
    .run()
    .await
}

#[cfg(test)]
mod test {
    use super::*;
}
