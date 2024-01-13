use diesel::Queryable;
use diesel::Insertable;
use diesel::sql_types::{Text, Timestamp, Integer};
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub publickey: String,
    pub createdat: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::login_action)]
pub struct LoginActionModel {
    pub id: i32,
    pub publickey: String,
    pub actiondata: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::signup_action)]
pub struct SignupActionModel {
    pub id: i32,
    pub publickey: String,
    pub actiondata: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub publickey: String,
    pub createdat: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::login_action)]
pub struct NewLoginActionModel {
    pub publickey: String,
    pub actiondata: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::signup_action)]
pub struct NewSignupActionModel {
    pub publickey: String,
    pub actiondata: chrono::NaiveDateTime,
}
