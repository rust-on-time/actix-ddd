use actix_web::{
    get, post,
    web::{self, Path},
    HttpResponse,
};
use log::error;

use crate::{
    application::{
        dto::NewUser,
        use_cases::{GetUserUseCase, RegisterUserUseCase},
    },
    infrastructure::repositories::PgUserRepository,
};

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
            HttpResponse::InternalServerError().body("Cannot find User with that email!")
        }
    }
}
