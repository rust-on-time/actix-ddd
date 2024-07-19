use async_trait::async_trait;

use crate::{application::dto::NewUser, domain::entities::User};

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Option<User>;
    async fn save(&self, user: &NewUser) -> Result<(), diesel::result::Error>;
}
