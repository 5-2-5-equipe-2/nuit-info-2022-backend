use serde::{Deserialize, Serialize};

pub mod scope{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    pub struct Scope {
        pub code: u16,
        pub name: &'static str
    }
    pub const ADMIN :Scope = Scope {
        code : 0,
        name : "admin"
    };
    pub const READ:Scope = Scope {
        code : 1,
        name :"read"
    };
    pub const WRITE:Scope = Scope {
        code : 2,
        name : "write"
    };
    pub const READ_WRITE:Scope = Scope {
        code : 3,
        name :"read and write"
    };
}


#[derive(Serialize, Deserialize)]
pub struct UserLog {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct UserRefresh{
    pub refresh: String
}

#[derive(Clone,Serialize, Deserialize, Default)]
pub struct TokenJWT {
    pub refresh: String,
    pub access : String,
    pub timestamp : u64
}