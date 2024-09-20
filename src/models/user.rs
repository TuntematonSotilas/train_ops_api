use serde::{Deserialize, Serialize};
use bson::serde_helpers::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct User {
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub _id: String,
    pub username: String,
    pub avatar: String,
}