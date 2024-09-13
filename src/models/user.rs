use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    #[serde(rename(deserialize = "_id"))]
    pub id: Bson,
    pub username: String,
    pub avatar: String,
}