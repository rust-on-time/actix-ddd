use actix_web::{
    get, post,
    web::{self, Path},
    HttpResponse,
};
use diesel::prelude::Insertable;
use log::error;
use serde::Deserialize;

use crate::{
    application::use_cases::{get_user::GetUserUseCase, register_user::RegisterUserUseCase},
    infrastructure::repositories::pg_user_repository::PgUserRepository,
    schema::users,
};

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[post("/")]
pub async fn register_user_handler(
    repo: web::Data<PgUserRepository>,
    input: web::Json<NewUser>,
) -> HttpResponse {
    match RegisterUserUseCase::new(repo.into_inner())
        .exec(&input.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error registering user! {:?}", e);
            HttpResponse::InternalServerError().body("Please try again...!")
        }
    }
}

#[get("/{email}")]
pub async fn find_user_by_email(
    repo: web::Data<PgUserRepository>,
    path: Path<(String,)>,
) -> HttpResponse {
    match GetUserUseCase::new(repo.into_inner())
        .exec(&path.into_inner().0)
        .await
    {
        Some(user) => HttpResponse::Ok().json(user),
        None => {
            error!("Cant find user email");
            HttpResponse::InternalServerError().body("Please Try again!")
        }
    }
}
