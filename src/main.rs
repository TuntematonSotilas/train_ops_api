use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use mongodb::Client;

mod models;
mod auth;
mod routes;

use auth::auth_validator::validator;
use routes::login::login;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let uri = "mongodb+srv://todo";

    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    /*let mut client_options = 
        ClientOptions::parse(uri)
        .await.unwrap();
    let server_api = ServerApi::builder().version(ServerApiVersion::V).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).unwrap();*/

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(login)
            .wrap(HttpAuthentication::with_fn(validator))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}