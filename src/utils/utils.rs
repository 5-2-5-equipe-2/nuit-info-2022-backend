use mysql::{Pool,PooledConn,Opts,Result};
use sha2::{Sha256, Sha512, Digest};
use jwt_simple::prelude::*;

pub struct ConnectInfo {
    pub host : &'static str,
    pub port : &'static str,
    pub user : &'static str,
    pub password : &'static str,
    pub db_name : &'static str
}

pub fn connect_database(connect_info:ConnectInfo) -> Result<Pool> {
    let url = format!("mysql://{}:{}@{}:{}/{}", connect_info.user,connect_info.password,connect_info.host,connect_info.port,connect_info.db_name);
    let pool = Pool::new(Opts::from_url(&url).unwrap());
    pool
}

pub fn crypt_password(password:String) -> Option<String>{
    let mut sha256 = Sha256::new();
    sha256.update(password);
    let mut crypt_password: String = format!("{:X}", sha256.finalize());
    Some(crypt_password.to_lowercase())
}

pub fn check_token(token:String) -> Option<String>{
    let xs: [u8; 32] = [241, 167, 179, 123, 41, 128, 25, 208, 162, 245, 241, 228, 24, 132, 163, 245, 102, 140, 140, 234, 235, 14, 90, 104, 15, 129, 8, 61, 174, 109, 250, 28];
    let key = HS256Key::from_bytes(&xs);
    let claims = key.verify_token::<NoCustomClaims>(&token, None);
    if claims.is_err(){
        return None;
    }
    Some(token)
}