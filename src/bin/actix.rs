use arch::infrastructure::servers::actix::run;

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run().await
}
