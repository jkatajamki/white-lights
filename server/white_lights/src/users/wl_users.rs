use chrono::{NaiveDateTime, Utc};
use crate::schema;
use crate::db;
use crate::diesel::{QueryDsl, RunQueryDsl, sql_query, ExpressionMethods};
use crate::diesel::sql_types::{Timestamp, Text};
use serde::Serialize;
use schema::wl_users::dsl::*;

#[derive(Queryable, Serialize)]
pub struct WLUser {
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub email: String,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct CreateUserRequest {
    pub email: String,
    pub user_secret: String,
}

pub fn db_get_users() -> Result<Vec<WLUser>, diesel::result::Error> {
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

pub fn db_get_user_by_email(email_req: String) -> Result<WLUser, diesel::result::Error> {
    let connection = db::db_connect();

    wl_users
        .filter(email.eq(email_req))
        .select((
            user_id,
            created_at,
            email,
            updated_at,
        ))
        .first(&connection)
}

pub fn db_create_user(create_user_req: CreateUserRequest) -> Result<WLUser, diesel::result::Error> {
    let connection = db::db_connect();

    let user_email_req = String::from(&create_user_req.email);

    // TODO: password cryptography

    let insert_user_query = "
        INSERT INTO wl_users
          (created_at, email, user_secret)
        VALUES ($1, $2, $3)
    ";

    let naive_now = Utc::now().naive_utc();

    let query = sql_query(insert_user_query)
        .bind::<Timestamp, _>(naive_now)
        .bind::<Text, _>(create_user_req.email)
        .bind::<Text, _>(create_user_req.user_secret);

    let result = query.execute(&connection);

    match result {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    db_get_user_by_email(user_email_req)
}
