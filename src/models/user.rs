use diesel::prelude::{Insertable, Queryable};

use crate::schema::users;

#[derive(Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

pub enum UserRole {
    Moderator,
    Admin,
}
