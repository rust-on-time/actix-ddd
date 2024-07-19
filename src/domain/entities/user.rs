use diesel::{deserialize::Queryable, Selectable};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}
