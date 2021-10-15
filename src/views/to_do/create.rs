use super::utils::return_state;
use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;
use actix_web::{HttpRequest, Responder};
use diesel::prelude::*;

pub async fn create(req: HttpRequest) -> impl Responder {
    let title = req.match_info().get("title").unwrap().to_string();
    let title_ref = title.clone();
    let token = JwtToken::decode_from_request(req).unwrap();

    let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&connection);
    }

    return return_state(&token.user_id);
}
