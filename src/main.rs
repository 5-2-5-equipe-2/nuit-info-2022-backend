use axum::{routing::get, routing::post, Router, Json,http::{header::HeaderMap, header::HeaderValue}, extract::Extension, middleware};
use std::{net::SocketAddr,sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use serde::{Deserialize, Serialize};
use mysql::PooledConn;

mod controllers;
use controllers::{auth::get_token,auth::get_refresh_token};

mod my_middleware;
use my_middleware::{my_middleware::{auth_middleware,nauth_middleware,Context}};

mod models;
mod utils;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let cors_auth = CorsLayer::new().allow_origin(Any);

    let unauth_endpoint = Router::new()
        .route("/", get(root))
        .route("/auth/token", post(get_token))
        .route("/auth/refresh", post(get_refresh_token))
        .route_layer(middleware::from_fn(nauth_middleware))
        .layer(cors);

    let auth_endpoint = Router::new()
        .route("/hello", get(auth_root))
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(cors_auth);

    let app = unauth_endpoint.merge(auth_endpoint);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(Extension(context): Extension<Context>) -> &'static str {
    "Hello, World!"
}

async fn auth_root(Extension(context): Extension<Context>) -> &'static str {
    println!("Hello world {}",context.current_user.unwrap().username);
    "Hello, World Auth!"
}
