use axum::{
    extract::Extension,
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
};
use mysql::prelude::Queryable;
use mysql::{Pool, PooledConn, Result, Row, Value};
use serde::{Deserialize, Serialize};

use jwt_simple::prelude::*;
use sha2::{Digest, Sha256, Sha512};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{
    auth::{TokenJWT, UserLog, UserRefresh},
    users::CurrentUser,
};

use crate::utils::{utils::check_token, utils::crypt_password};

pub fn add_user(conn: &mut PooledConn, user: CurrentUser) {
    let row: Result<Vec<Row>> = conn.exec(
        "INSERT INTO user (id,username,password,scopes) VALUES (0,?,?,?)",
        (
            user.username,
            crypt_password(user.password).unwrap(),
            user.scopes,
        ),
    );
}

pub fn get_user_by_id(conn: &mut PooledConn, id: u32) -> Option<CurrentUser> {
    let row: Result<Vec<Row>> = conn.exec("SELECT * FROM user WHERE id=?", (id,));

    match row {
        Ok(ref v) => {
            if v.len() > 0 {
                Some(CurrentUser {
                    id: v[0].get::<u32, &str>("id").unwrap(),
                    username: v[0].get::<String, &str>("username").unwrap(),
                    password: v[0].get::<String, &str>("password").unwrap(),
                    scopes: v[0].get::<u8, &str>("scopes").unwrap(),
                })
            } else {
                None
            }
        }
        Err(e) => None,
    }
}

pub fn get_user_by_user_log(conn: &mut PooledConn, user_log: UserLog) -> Option<CurrentUser> {
    let row: Result<Vec<Row>> = conn.exec(
        "SELECT * FROM user WHERE username=? AND password=?",
        (user_log.username, user_log.password),
    );

    match row {
        Ok(ref v) => {
            if v.len() > 0 {
                Some(CurrentUser {
                    id: v[0].get::<u32, &str>("id").unwrap(),
                    username: v[0].get::<String, &str>("username").unwrap(),
                    password: v[0].get::<String, &str>("password").unwrap(),
                    scopes: v[0].get::<u8, &str>("scopes").unwrap(),
                })
            } else {
                None
            }
        }
        Err(e) => None,
    }
}

pub fn get_user_by_token(conn: &mut PooledConn, auth_token: String) -> Option<CurrentUser> {
    let result_check = check_token(auth_token.clone());
    if result_check.is_none() {
        return None;
    }
    let row: Result<Vec<Row>> =
        conn.exec("SELECT * FROM oauth_token WHERE access=?", (auth_token,));
    match row {
        Ok(ref v) => {
            if v.len() > 0 {
                get_user_by_id(conn, v[0].get::<u32, &str>("id_user").unwrap())
            } else {
                None
            }
        }
        Err(e) => None,
    }
}

pub fn get_user_by_refresh_token(conn: &mut PooledConn, auth_token: String) -> Option<CurrentUser> {
    let result_check = check_token(auth_token.clone());
    if result_check.is_none() {
        return None;
    }
    let row: Result<Vec<Row>> =
        conn.exec("SELECT * FROM oauth_token WHERE refresh=?", (auth_token,));
    match row {
        Ok(ref v) => {
            if v.len() > 0 {
                get_user_by_id(conn, v[0].get::<u32, &str>("id_user").unwrap())
            } else {
                None
            }
        }
        Err(e) => None,
    }
}
