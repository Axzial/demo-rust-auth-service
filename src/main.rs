mod resource;
mod utils;

use rocket::{Build, catch, catchers, get, launch, Rocket, routes};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::utils::jwt_utils;

use crate::resource::auth_resource;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDTO {
    pub message: String,
}

#[catch(404)]
fn not_found() -> Json<ErrorDTO> {
    return Json(
        ErrorDTO {
            message: "Not found".to_string(),
        }
    )
}

#[catch(500)]
fn internal_server_error() -> Json<ErrorDTO> {
    return Json(
        ErrorDTO {
            message: "Internal server error".to_string(),
        }
    )
}

#[get("/")]
fn index() -> String {
    return jwt_utils::generate("test".to_string());
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index])
        .mount("/auth", routes![auth_resource::login, auth_resource::register])
}
