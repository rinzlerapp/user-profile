use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: String,
    pub username: String,
    pub fullname: String,
    pub profile_image: Option<String>,
}