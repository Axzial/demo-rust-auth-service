use rocket::{routes, get, launch, Rocket, Build, Response, catch, catchers};
use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[path = "utils/jwt_utils.rs"] mod jwt_utils;

// login dto
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDTO {
    pub token: String,
}

#[catch(404)]
fn not_found() -> &'static str {
    "Sorry, this route does not exist."
}

#[catch(500)]
fn internal_server_error() -> &'static str {
    "Whoops! Looks like we messed up."
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
}
