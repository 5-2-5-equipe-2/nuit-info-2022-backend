use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::jwt::validate_token;

pub async fn auth_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let v_token: Vec<&str> = auth_header.split(' ').collect();
    if v_token.len() >= 2 {
        if v_token[0] == "Bearer" {
            let token_result = validate_token(v_token[1]);
            if token_result.await.is_ok() {
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
