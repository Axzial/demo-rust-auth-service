use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDTO {
    pub token: String,
}

#[post("/login", data = "<login>")]
pub fn login(login: Json<LoginDTO>) -> Json<TokenDTO> {
    return Json(
        TokenDTO {
            token: "test".to_string(),
        }
    )
}

#[post("/register", data = "<register>")]
pub fn register(register: Json<RegisterDTO>) -> Json<TokenDTO> {
    return Json(
        TokenDTO {
            token: "test".to_string(),
        }
    )
}
