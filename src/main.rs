use axum::{routing::get, routing::post, Router, Json,http::{header::HeaderMap, header::HeaderValue}, extract::Extension, middleware};
use std::{net::SocketAddr,sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use serde::{Deserialize, Serialize};
use mysql::PooledConn;

mod jwt;
use jwt::{UserLog,UserRefresh,TokenJWT,CurrentUser,generate_token_user,generate_refresh_token_user};

mod database;
use database::{ConnectInfo,connect_database};

mod my_middleware;
use my_middleware::{auth_middleware,nauth_middleware,Context};


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
//Extension(db_connection): Extension<PooledConn>, 
async fn get_token(Extension(context): Extension<Context>, Json(req): Json<UserLog>) -> Json<TokenJWT> {
    println!("start_mission, req: {:?}", req.username);
    let rsp = generate_token_user(&mut context.db_connection.unwrap(),req).unwrap();
    rsp.into()
}

async fn get_refresh_token(Extension(context): Extension<Context>,Json(req): Json<UserRefresh>) -> Json<TokenJWT> {
    let rsp = generate_refresh_token_user(&mut context.db_connection.unwrap(),req.refresh).unwrap();
    rsp.into()
}