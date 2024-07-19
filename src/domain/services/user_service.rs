use crate::{
    domain::{entities::User, repositories::UserRepository},
    presentation::handlers::NewUser,
};

pub struct UserService<T>
where
    T: UserRepository,
{
    user_repo: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(new_repo: T) -> Self {
        UserService {
            user_repo: new_repo,
        }
    }

    pub async fn register(&self, user: &NewUser) -> Result<(), diesel::result::Error> {
        self.user_repo.save(user).await
    }

    pub async fn find_by_email(&self, email: &str) -> Option<User> {
        self.user_repo.find_by_email(email).await
    }
}
