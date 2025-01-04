use diesel::prelude::{Insertable, Queryable};

use crate::schema::boards;

#[derive(Queryable, Insertable)]
#[diesel(table_name = boards)]
pub struct Board {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
