use actix_web::{post, web, Result, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use mongodb::{bson::doc, Client, Collection};

use crate::{models::user::User, DB_NAME};


const COLL_NAME: &str = "users";

#[post("/login")]
pub async fn login(creds: BasicAuth, client: web::Data<Client>) -> Result<impl Responder> {
    
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);

    let user = match collection.find_one(doc! { "username": &creds.user_id() }).await {
        Ok(Some(user)) => user,
        _ => User::default()
    };

    Ok(web::Json(user))
}

