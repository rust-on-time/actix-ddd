use diesel::Insertable;
use serde::Deserialize;

use crate::schema::users;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}
