use actix_web::dev::ServiceRequest;
use super::jwt;

pub fn check_password(password: String) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(password) {
        Ok(_) => Ok(String::from("passed")),
        Err(message) => Err(message)
    }
}

pub fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match request.headers().get("user-token") {
        Some(token) => {
            match token.to_str(){
                Ok(processed_pasword) => Ok(String::from(processed_pasword)),
                Err(_) => Err("there was an error processing token")
            }
        },
        None => Err("there is no token")
    }
}
