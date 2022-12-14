use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    middleware,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
#[cfg(debug_assertions)]
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use auth_middleware::auth_middleware;
use entity::async_graphql;
use graphql::schema::{build_schema, build_schema_auth, AppSchema, AppSchemaAuth};
use migration::{Migrator, MigratorTrait};

use crate::db::Database;

mod auth_middleware;
mod db;
mod graphql;
mod jwt;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_handler_auth(
    schema: Extension<AppSchemaAuth>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

#[tokio::main]
pub async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let db = Database::new().await;
    let conn = db.get_connection();
    let cors = CorsLayer::new().allow_origin(Any);

    Migrator::up(&conn.clone(), None).await.unwrap();

    let schema = build_schema(conn.clone()).await;
    let schema_auth = build_schema_auth(conn.clone()).await;

    let unauth_endpoint = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(Extension(schema))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let auth_endpoint = Router::new()
        .route("/api/auth/graphql", post(graphql_handler_auth))
        //.route_layer(middleware::from_fn(auth_middleware))
        .layer(Extension(schema_auth))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let app = unauth_endpoint.merge(auth_endpoint);

    println!("Playground: http://localhost:3000/api/graphql");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
