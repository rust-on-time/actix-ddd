use diesel::deserialize::Queryable;
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}
