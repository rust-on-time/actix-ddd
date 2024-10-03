# DDD Arch in Rust


## Usage

1. clone repo

```sh
git clone <repo-url>
```
2. run postgres with docker

```sh
docker-compose up -d
```

3. setup db
```
diesel setup
```

4. run migration

```
diesel migrate run
```

5. run app

```
cargo run
```

## How to re-create this

### Setup Schema and Migration

1. create migration for `User`

```
diesel migration generate create_user_table
```

2. Update up.sql

### Build our Domain layer

1. Create Entity for `User`

important to note that we need the ff: traits

```rust
use diesel::deserialize::Queryable;
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, Clone)]
``



2. Create Repository on Domain layer for User

`/src/domain/repositories/user_repository.rs`


create a trait with no implementation of methods

e.g. 

```rust
#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Option<User>;
    async fn save(&self, user: &NewUser) -> Result<(), diesel::result::Error>;
}
```

3. Create a Service on Domain Layer for User

`src/domain/services/user_service.rs`

this would just consume any repository associated with this service

e.g. `UserRepository` 

we can also use other stuff like `NewUser` DTO

```rust
use crate::{
    application::dto::NewUser,
    domain::{entities::User, repositories::UserRepository},
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

```

### Build our Application Layer

1. Build UseCases in our Application Layer
It is important to note that on UseCases
we would consume a Repository  , a Service , an Entity

example:
`/src/application/use_cases/get_user.rs`


```rust
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
```

2. Create Dto for `NewUser` under Application Layer

`src/application/dto/new_user_dto.rs`

```rust
use diesel::Insertable;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}
```

This is somewhat similar with an Entity except this dont have id
also it can have an `Insertable` trait
also a entity uses `Serialize` while a Dto uses `Deserialized` 

### Build our Infrastructure Layer

1. create a db fn that returns a db pool

`src/infrastructure/db/connection.rs`

```rust
use diesel::{r2d2::ConnectionManager, PgConnection};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(db_url: &str) -> DBPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB Pool")
}
```

2. Create PgUserRepository , impl UserRepository

Note: important to note that this impl of UserRepository would also have a `DbPool` as sole field

```rust
#[derive(Clone)]
pub struct PgUserRepository {
    pool: DBPool,
}
```

also since we are using actix the impl would be wrapped with Arc<T>

```rust
#[async_trait]
impl UserRepository for Arc<PgUserRepository> {
    async fn find_by_email(&self, input_email: &str) -> Option<User> {
        users
            .filter(email.eq(input_email))
            .first::<User>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading user")
    }
    async fn save(&self, user: &NewUser) -> Result<(), diesel::result::Error> {
        diesel::insert_into(table)
            .values(user)
            .execute(&mut self.pool.get().unwrap())?;

        Ok(())
    }
}
```


### Build our Presentation Layer

This most likely invole our routes and its handler

1. create a `user_handler` 

This part of code is very specific with Actix Route Handler Fn

```rust

use actix_web::{
    post,
    web,
    HttpResponse,
};

use crate::{
    application::{
        dto::NewUser,
        use_cases::RegisterUserUseCase,
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
```

2. create `user_routes`

```rust
use actix_web::web::{self, ServiceConfig};

use crate::presentation::handlers::{find_user_by_email, register_user_handler};

pub fn routes(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api/v1/user")
            .service(register_user_handler)
            .service(find_user_by_email),
    );
}
```

### Implement an Actix web server on bin directory

Note: in theory we can add other server on `src/bin` e.g. axum , tonic etc.

```rust
use actix_web::{middleware::Logger, web, App, HttpServer};
use arch::{infrastructure::repositories::PgUserRepository, presentation::routes};
use log::info;

use env_logger::Env;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run().await
}

```