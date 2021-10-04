use actix_web::dev::ServiceRequest;

fn check_password(password: String) -> Result<String, &'static str> {
    if password == "token" {
        return Ok(password);
    }
    return Err("token not authorized");
}

fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
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

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match extract_header_token(request) {
        Ok(token) => check_password(token),
        Err(message) => Err(message)
    }
}
