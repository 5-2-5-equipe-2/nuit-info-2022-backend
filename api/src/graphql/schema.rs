use async_graphql::{EmptySubscription, Schema};

use entity::async_graphql;
use migration::{Migrator, MigratorTrait};

use crate::{
    db::Database,
    graphql::{mutation::Mutation, mutation::MutationAuth, query::Query, query::QueryAuth},
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaAuth = Schema<QueryAuth, MutationAuth, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema(db: Database) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}

pub async fn build_schema_auth(db:Database) -> AppSchemaAuth {
    Schema::build(QueryAuth::default(), MutationAuth::default(), EmptySubscription)
        .data(db)
        .finish()
}