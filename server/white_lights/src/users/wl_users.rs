use chrono::{NaiveDateTime};
use crate::schema;
use crate::db;
use crate::diesel::RunQueryDsl;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct WLUser {
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub email: String,
    pub user_secret: String,
    pub updated_at: NaiveDateTime,
}

pub fn db_get_users() -> Result<Vec<WLUser>, diesel::result::Error> {
    use schema::wl_users::dsl::*;

    let connection = db::db_connect();

    wl_users.load::<WLUser>(&connection)
}
