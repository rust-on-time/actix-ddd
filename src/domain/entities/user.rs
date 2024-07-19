use diesel::deserialize::Queryable;
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, Clone)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}
