use serde::{Deserialize, Serialize};

#[derive(Clone,Serialize, Deserialize)]
pub struct CurrentUser { 
    pub id : u32,
    pub username: String,
    pub password: String,
    pub scopes:u8
}