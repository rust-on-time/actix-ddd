use crate::{infrastructure::repositories::PgUserRepository, presentation::routes};
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

pub async fn run() -> std::io::Result<()> {
    let repo = PgUserRepository::new();
    let app_data = web::Data::new(repo);

    info!("starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routes::user_routes::routes)
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
}
