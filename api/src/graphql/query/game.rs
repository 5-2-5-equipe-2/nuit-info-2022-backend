use async_graphql::{Context, Object, Result};

use crate::graphql::query::game;
use entity::async_graphql;
use entity::game::Model;
use graphql_core::game::Query;
use migration::sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct GameQuery;

#[Object]
impl GameQuery {
    async fn get_game_by_user_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Query::find_game_by_user_id(conn, id)
            .await
            .map_err(|e| e.to_string())?)
    }
}
