use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use std::sync::Arc;

use crate::{
    domain::{entities::User, repositories::UserRepository},
    infrastructure::db::connection::{establish_connection, DBPool},
    schema::{users::dsl::users, users::email, users::table},
};

#[derive(Clone)]
pub struct PgUserRepository {
    pool: DBPool,
}

impl Default for PgUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl PgUserRepository {
    pub fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        PgUserRepository {
            pool: establish_connection(&db_url),
        }
    }
}
#[async_trait]
impl UserRepository for Arc<PgUserRepository> {
    async fn find_by_email(&self, input_email: &str) -> Option<User> {
        users
            .filter(email.eq(input_email))
            .first::<User>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading user")
    }
    async fn save(&self, user: &User) -> Result<(), diesel::result::Error> {
        diesel::insert_into(table)
            .values(user)
            .execute(&mut self.pool.get().unwrap())
            .unwrap();

        Ok(())
    }
}
