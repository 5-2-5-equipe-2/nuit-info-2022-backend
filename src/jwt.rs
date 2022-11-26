use serde::{Deserialize, Serialize};
use axum::{
    Router,
    http::{Request, StatusCode, header},
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::Extension,
};
use mysql::{Pool,PooledConn,Result,Row,Value};
use mysql::prelude::Queryable;

use sha2::{Sha256, Sha512, Digest};
use jwt_simple::prelude::*;

use std::time::{SystemTime, UNIX_EPOCH};

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

#[derive(Clone,Serialize, Deserialize)]
pub struct CurrentUser { 
    pub id : u32,
    pub username: String,
    pub password: String,
    pub scopes:u8
}

pub fn generate_token_pair() -> Option<TokenJWT> {
    let xs: [u8; 32] = [241, 167, 179, 123, 41, 128, 25, 208, 162, 245, 241, 228, 24, 132, 163, 245, 102, 140, 140, 234, 235, 14, 90, 104, 15, 129, 8, 61, 174, 109, 250, 28];
    let key = HS256Key::from_bytes(&xs);
    let claims = Claims::create(Duration::from_hours(1));
    let token = key.authenticate(claims).ok()?;
    let refreshclaims = Claims::create(Duration::from_hours(140));
    let refreshtoken = key.authenticate(refreshclaims).ok()?;
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    Some(TokenJWT{
        refresh : refreshtoken,
        access : token,
        timestamp: since_the_epoch.as_secs()+3600
    })
}

pub fn generate_token_user(db_connection: &mut Pool, user:UserLog) -> Option<TokenJWT> {
    let db_conn=db_connection.get_conn();
    match db_conn {
        Ok(mut v) => {
            let user_find = get_user_by_user_log(&mut v,UserLog {
                username : user.username,
                password : crypt_password(user.password).unwrap()
            });
            if user_find.is_some() {
                let tokens = generate_token_pair().unwrap();
                insert_token(&mut v,tokens.clone(),user_find.unwrap().id);
                Some(tokens)
            }else{
                None
            }
        },
        Err(e) => {
            return None
        }
    }
}

pub fn generate_refresh_token_user(db_connection: &mut Pool, refresh_token:String) -> Option<TokenJWT> {
    let db_conn=db_connection.get_conn();
    match db_conn {
        Ok(mut v) => {
            let user_find = get_user_by_refresh_token(&mut v,refresh_token.clone());
            if user_find.is_some() {
                let tokens = generate_token_pair().unwrap();
                insert_token(&mut v,tokens.clone(),user_find.unwrap().id);
                delete_token_by_refresh_token(&mut v, refresh_token);
                Some(tokens)
            }else{
                None
            }
        },
        Err(e) => {
            return None
        }
    }
}

pub async fn authorize_current_user(db_connection: &mut Pool,auth_token: &str) -> Option<CurrentUser> {
    let db_conn=db_connection.get_conn();
    match db_conn {
        Ok(mut v) => {           
            println!("auth {}",auth_token);
            let v_token: Vec<&str> = auth_token.split(' ').collect();
            if v_token.len()>= 2 {
                if v_token[0] == "Bearer" {
                    println!("Token {}", v_token[1]);
                    get_user_by_token(&mut v,v_token[1].to_string())
                } else {
                    None
                }
            }else{
                None
            }
        },
        Err(e) => {
            return None
        }
    }
}

pub fn add_user(conn:&mut PooledConn, user:CurrentUser) {
    let row:Result<Vec<Row>> = conn.exec("INSERT INTO user (id,username,password,scopes) VALUES (0,?,?,?)", (user.username,crypt_password(user.password).unwrap(),user.scopes,));
}

pub fn get_user_by_id( conn:&mut PooledConn, id:u32) -> Option<CurrentUser> {
    let row:Result<Vec<Row>> = conn.exec("SELECT * FROM user WHERE id=?", (id,));

    match row {
        Ok(ref v) => {
            if v.len()>0 {
                Some(CurrentUser{
                    id : v[0].get::<u32,&str>("id").unwrap(),
                    username :v[0].get::<String,&str>("username").unwrap(),
                    password :v[0].get::<String,&str>("password").unwrap(),
                    scopes : v[0].get::<u8,&str>("scopes").unwrap()
                })
            }else{
                None
            }
        },
        Err(e) => {
            None
        }
    }
}

pub fn get_user_by_user_log( conn:&mut PooledConn, user_log : UserLog) -> Option<CurrentUser> {
    let row:Result<Vec<Row>> = conn.exec("SELECT * FROM user WHERE username=? AND password=?", (user_log.username,user_log.password,));

    match row {
        Ok(ref v) => {
            if v.len()>0 {
                Some(CurrentUser{
                    id : v[0].get::<u32,&str>("id").unwrap(),
                    username :v[0].get::<String,&str>("username").unwrap(),
                    password :v[0].get::<String,&str>("password").unwrap(),
                    scopes : v[0].get::<u8,&str>("scopes").unwrap()
                })
            }else{
                None
            }
        },
        Err(e) => {
            None
        }
    }
}

pub fn insert_token( conn:&mut PooledConn, tokens: TokenJWT, id:u32){
    let row:Result<Vec<Row>> =conn.exec("INSERT INTO oauth_token (id_user,access,refresh,timeout) VALUES (?,?,?,?)", (id,tokens.access,tokens.refresh,tokens.timestamp,));
}

pub fn get_user_by_token(conn:&mut PooledConn, auth_token: String) -> Option<CurrentUser> {
    let result_check=check_token(auth_token.clone());
    if result_check.is_none(){
        return None;
    }
    let row:Result<Vec<Row>> = conn.exec("SELECT * FROM oauth_token WHERE access=?", (auth_token,));
    match row {
        Ok(ref v) => {
            if v.len()>0 {
                get_user_by_id(conn,v[0].get::<u32,&str>("id_user").unwrap())
            }else{
                None
            }
        },
        Err(e)=>{
            None
        }
    }
}

pub fn get_user_by_refresh_token(conn:&mut PooledConn, auth_token: String) -> Option<CurrentUser> {
    let result_check=check_token(auth_token.clone());
    if result_check.is_none(){
        return None;
    }
    let row:Result<Vec<Row>> = conn.exec("SELECT * FROM oauth_token WHERE refresh=?", (auth_token,));
    match row {
        Ok(ref v) => {
            if v.len()>0 {
                get_user_by_id(conn,v[0].get::<u32,&str>("id_user").unwrap())
            }else{
                None
            }
        },
        Err(e)=>{
            None
        }
    }
}

pub fn delete_token_by_refresh_token(conn:&mut PooledConn, refresh_token: String) {
    let row:Result<Vec<Row>> = conn.exec("DELETE FROM oauth_token WHERE refresh=?", (refresh_token,));
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