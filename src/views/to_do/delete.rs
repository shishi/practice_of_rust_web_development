use super::utils::return_state;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state = read_file(String::from("./state.json"));
    let title = to_do_item.title.clone();
    let status = to_do_item.status.clone();

    match to_do_factory(&status, title) {
        Err(_) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("delete"), &state),
    }

    return HttpResponse::Ok().json(return_state());
}
