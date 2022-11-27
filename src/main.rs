use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Extension, http::{header::HeaderMap, header::HeaderValue}, Json, middleware, Router, routing::get, routing::post};
use mysql::PooledConn;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use controllers::{auth::get_refresh_token, auth::get_token};
use my_middleware::{my_middleware::{auth_middleware, Context, nauth_middleware}};

mod controllers;
mod my_middleware;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
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
    let addr_str = std::env::var("RUST_SOCKET_ADDR").expect("RUST_SOCKET_ADDR must be set");
    let addr: SocketAddr = addr_str.parse().unwrap();
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
