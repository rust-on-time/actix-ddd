use actix_web::web::{self, ServiceConfig};

use crate::presentation::handlers::{find_user_by_email, register_user_handler};

pub fn routes(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api/v1/user")
            .service(register_user_handler)
            .service(find_user_by_email),
    );
}
