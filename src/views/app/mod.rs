use actix_web::web;
mod content_loader;
mod items;
mod login;
mod logout;
use super::path::Path;

pub fn app_factoty(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/"),
    };
    app.route(
        &base_path.define(String::from("")),
        web::get().to(items::items),
    );
    app.route(
        &base_path.define(String::from("login")),
        web::get().to(login::login),
    );
    app.route(
        &base_path.define(String::from("logout")),
        web::get().to(logout::logout),
    );
}
