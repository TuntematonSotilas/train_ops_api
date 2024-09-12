use actix_web::{dev::ServiceRequest, error, web, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use mongodb::{bson::doc, Client, Collection};

use crate::models::user::User;

const DB_NAME: &str = "TrainOps";
const COLL_NAME: &str = "users";

pub async fn validator(
    req: ServiceRequest,
    creds: BasicAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    
    let client = req.app_data::<web::Data<Client>>().unwrap();

    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.find_one(doc! { "username": &creds.user_id() }).await {
        Ok(Some(_user)) => Ok(req),
        Ok(None) => {
            Err((error::ErrorUnauthorized(""), req))
        }
        Err(e) => Err((error::ErrorInternalServerError(e.to_string()), req)),
    }

}
