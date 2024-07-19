use crate::domain::{entities::User, repositories::UserRepository, services::UserService};

pub struct GetUserUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> GetUserUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        GetUserUseCase { user_service }
    }

    pub async fn exec(&self, email: &str) -> Option<User> {
        self.user_service.find_by_email(email).await
    }
}
