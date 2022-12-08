use std::collections::HashMap;

use chrono::{Utc};
use jsonwebtoken::{Algorithm, DecodingKey, encode, EncodingKey, Header};

const JWT_SECRET: &str = "secret";
const JWT_EXPIRATION: i64 = 128000;

fn default_headers() -> Header {
    let mut headers = Header::default();
    headers.alg = Algorithm::HS256;
    return headers;
}

fn default_claims(sub: String) -> HashMap<&'static str, String> {
    let mut claims: HashMap<&str, String> = HashMap::new();
    claims.insert("sub", sub);
    claims.insert("exp", (Utc::now().timestamp() + JWT_EXPIRATION).to_string());
    return claims;
}

fn default_validator() -> jsonwebtoken::Validation {
    let mut validator = jsonwebtoken::Validation::default();
    validator.algorithms = Vec::from([Algorithm::HS256]);
    return validator;
}

pub(crate) fn generate(sub: String) -> String {
    let claims = default_claims(sub);
    let token: String = encode(&default_headers(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref())).unwrap();
    return token;
}

pub(crate) fn validate(token: String) -> bool {
    let validator = default_validator();
    let token_data = jsonwebtoken::decode::<HashMap<String, String>>(&token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &validator);
    return token_data.is_ok();
}

fn get_sub(token: String) -> String {
    let validator = default_validator();
    let token_data = jsonwebtoken::decode::<HashMap<String, String>>(&token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &validator).unwrap();
    return token_data.claims.get("sub").unwrap().to_string();
}

fn is_expired(token: String) -> bool {
    let validator = default_validator();
    let token_data = jsonwebtoken::decode::<HashMap<String, String>>(&token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &validator).unwrap();
    let exp = token_data.claims.get("exp").unwrap().parse::<i64>().unwrap();
    return exp < Utc::now().timestamp();
}


