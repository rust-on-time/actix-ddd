use async_trait::async_trait;

use crate::domain::entities::User;

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Option<User>;
    async fn save(&self, user: &User) -> Result<(), diesel::result::Error>;
}
