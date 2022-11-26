use serde::{Deserialize, Serialize};
use axum::{
    Router,
    http::{Request, StatusCode, header},
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::Extension,
    Json,
};
use mysql::{Pool,PooledConn,Result,Row,Value};
use mysql::prelude::Queryable;

use sha2::{Sha256, Sha512, Digest};
use jwt_simple::prelude::*;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{
    auth::{UserLog,UserRefresh,TokenJWT,},
    users::CurrentUser
};

use crate::controllers::{
    users::{get_user_by_refresh_token,get_user_by_token,get_user_by_user_log}
};

use crate::utils::{
    utils::crypt_password,
    utils::check_token
};

use crate::my_middleware::{
    my_middleware::Context
};

pub async fn get_token(Extension(context): Extension<Context>, Json(req): Json<UserLog>) -> Json<TokenJWT> {
    let rsp = generate_token_user(&mut context.db_connection.unwrap(),req).unwrap();
    rsp.into()
}

pub async fn get_refresh_token(Extension(context): Extension<Context>,Json(req): Json<UserRefresh>) -> Json<TokenJWT> {
    let rsp = generate_refresh_token_user(&mut context.db_connection.unwrap(),req.refresh).unwrap();
    rsp.into()
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

pub fn insert_token( conn:&mut PooledConn, tokens: TokenJWT, id:u32){
    let row:Result<Vec<Row>> =conn.exec("INSERT INTO oauth_token (id_user,access,refresh,timeout) VALUES (?,?,?,?)", (id,tokens.access,tokens.refresh,tokens.timestamp,));
}

pub fn delete_token_by_refresh_token(conn:&mut PooledConn, refresh_token: String) {
    let row:Result<Vec<Row>> = conn.exec("DELETE FROM oauth_token WHERE refresh=?", (refresh_token,));
}