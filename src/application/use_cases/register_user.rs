use crate::domain::{
    entities::User, repositories::UserRepository, services::user_service::UserService,
};

pub struct RegisterUserUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> RegisterUserUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        RegisterUserUseCase { user_service }
    }

    pub async fn get(&self, user: &User) -> Result<(), diesel::result::Error> {
        self.user_service.register(user).await
    }
}
