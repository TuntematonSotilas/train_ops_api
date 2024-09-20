use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use mongodb::Client;
use load_dotenv::load_dotenv;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod models;
mod auth;
mod routes;

const DB_NAME: &str = "TrainOps";

use auth::auth_validator::validator;
use routes::login::login;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .without_time()
        .init();

    load_dotenv!();
    let uri = env!("DATABASE_URL");
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default().log_target("@"))
            .wrap(HttpAuthentication::with_fn(validator))
            .wrap(cors)
            .app_data(web::Data::new(client.clone()))
            .service(login)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}