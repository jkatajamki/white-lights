use chrono::{NaiveDateTime};
use crate::schema;
use crate::db;
use crate::diesel::{QueryDsl, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct WLUser {
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub email: String,
    pub updated_at: NaiveDateTime,
}

pub struct CreateUserRequest {
    pub email: String,
    pub user_secret: String,
}

pub fn db_get_users() -> Result<Vec<WLUser>, diesel::result::Error> {
    use schema::wl_users::dsl::*;

    let connection = db::db_connect();

    wl_users
        .select((
            user_id,
            created_at,
            email,
            updated_at,
        ))
        .load::<WLUser>(&connection)
}
