use async_graphql::{Context, Object, Result};

use crate::graphql::query::questions;
use entity::async_graphql;
use entity::questions::Model;
use graphql_core::questions::Query;
use migration::sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct QuestionsQuery;

#[Object]
impl QuestionsQuery {
    async fn get_question_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Query::find_question_by_id(conn, id)
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_random_question(&self, ctx: &Context<'_>) -> Result<Option<Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Query::find_random_question(conn)
            .await
            .map_err(|e| e.to_string())?)
    }
}
