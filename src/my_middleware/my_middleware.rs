use axum::{
    Router,
    http::{Request, StatusCode, header},
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::Extension,
};
use mysql::{Pool,PooledConn};
use mysql::prelude::Queryable;

use crate::models::users::{CurrentUser};
use crate::controllers::auth::{authorize_current_user};

use crate::utils::utils::{connect_database};

#[derive(Clone)]
pub struct Context {
    pub db_connection : Option<Pool>,
    pub current_user : Option<CurrentUser>
}


pub async fn auth_middleware<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let mut conn = connect_database();
    if conn.is_ok() {
    if let Some(current_user) = authorize_current_user(conn.as_mut().unwrap(),auth_header).await {
            // insert the current user into a request extension so the handler can
                // extract it
        let context : Context = Context {
                    db_connection : Some(conn.unwrap()),
                    current_user : Some(current_user)
        };
        req.extensions_mut().insert(context);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }


}

pub async fn nauth_middleware<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let conn = connect_database();
    if conn.is_ok() {
            let context : Context = Context {
                db_connection : Some(conn.unwrap()),
                current_user : None
            };
            req.extensions_mut().insert(context);
            Ok(next.run(req).await)
        }else{
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
}