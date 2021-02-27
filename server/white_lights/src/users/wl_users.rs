use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use schema::wl_users::dsl::*;
use crate::schema;
use crate::db;
use crate::diesel::{QueryDsl, RunQueryDsl, sql_query, ExpressionMethods};
use crate::diesel::sql_types::{Timestamp, Text};
use crate::diesel::result::{QueryResult, Error as DieselError};

#[derive(Queryable, Serialize, Debug)]
pub struct WLUser {
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub email: String,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub create_user_email: String,
    pub create_user_secret: String,
}

pub fn db_get_users(connection: &db::DbConnection) -> Result<Vec<WLUser>, DieselError> {
    wl_users
        .select((
            user_id,
            created_at,
            email,
            updated_at,
        ))
        .load::<WLUser>(connection)
}

pub fn db_get_user_by_email(connection: &db::DbConnection, email_req: String) -> QueryResult<WLUser> {
    wl_users
        .filter(email.eq(email_req))
        .select((
            user_id,
            created_at,
            email,
            updated_at,
        ))
        .first(connection)
}

pub fn db_create_user(connection: &db::DbConnection, create_user_req: CreateUserRequest) -> Result<WLUser, DieselError> {
    let user_email_req = String::from(&create_user_req.create_user_email);

    // TODO: password cryptography

    let insert_user_query = "
        INSERT INTO wl_users
          (created_at, email, user_secret)
        VALUES ($1, $2, $3)
    ";

    let naive_now = Utc::now().naive_utc();

    let query = sql_query(insert_user_query)
        .bind::<Timestamp, _>(naive_now)
        .bind::<Text, _>(create_user_req.create_user_email)
        .bind::<Text, _>(create_user_req.create_user_secret);

    let result = query.execute(connection);

    match result {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    db_get_user_by_email(connection, user_email_req)
}

pub fn handle_registration(connection: db::DbConnection, create_user_req: CreateUserRequest) -> Result<WLUser, String> {
    let db_conn = &connection;

    let user_email_req = String::from(&create_user_req.create_user_email);

    let user_by_email = db_get_user_by_email(db_conn, user_email_req);

    match user_by_email {
        Ok(_) => {
            return Err(String::from("Requested email is already taken"));
        },
        Err(error) => match error {
            DieselError::NotFound => (),
            any_other_error => {
                return Err(any_other_error.to_string());
            }
        },
    };

    db_create_user(db_conn, create_user_req).map_err(|e: DieselError| e.to_string())
}
