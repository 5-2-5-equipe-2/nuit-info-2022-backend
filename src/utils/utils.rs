use jwt_simple::prelude::*;
use mysql::{Opts, Pool, PooledConn, Result};
use sha2::{Digest, Sha256, Sha512};

use crate::models::auth::TokenData;

pub fn connect_database() -> Result<Pool> {
    dotenv::dotenv().ok();
    let url = std::env::var("RUST_DATABASE_URL").expect("RUST_DATABASE_URL must be set");
    let pool = Pool::new(url.as_str());
    pool
}

pub fn crypt_password(password: String) -> Option<String> {
    let mut sha256 = Sha256::new();
    sha256.update(password);
    let mut crypt_password: String = format!("{:X}", sha256.finalize());
    Some(crypt_password.to_lowercase())
}

pub fn check_token(token: String) -> Option<String> {
    let xs: [u8; 32] = [
        241, 167, 179, 123, 41, 128, 25, 208, 162, 245, 241, 228, 24, 132, 163, 245, 102, 140, 140,
        234, 235, 14, 90, 104, 15, 129, 8, 61, 174, 109, 250, 28,
    ];
    let key = HS256Key::from_bytes(&xs);
    let claims = key.verify_token::<TokenData>(&token, None);
    if claims.is_err() {
        return None;
    }
    Some(token)
}
