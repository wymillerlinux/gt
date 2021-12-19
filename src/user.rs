use serde_derive::{Deserialize, Serialize};

pub struct User;

pub struct MutlipleUsers {
    pub data: Vec<UserResponse>,
    pub ok: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: i64,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    pub language: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    pub login: String,
    pub restricted: bool,
}

impl User {
    pub fn new() -> User {
        User {}
    }
}