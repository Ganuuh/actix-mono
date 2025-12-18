use crate::db::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,       // Changed from i64 to i32 to match Int4
    pub name: String,  // Matches Varchar
    pub email: String, // Matches Varchar
}

#[derive(Debug, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
