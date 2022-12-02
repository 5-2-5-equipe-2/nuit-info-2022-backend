use async_graphql::{Context, Object, Result};

use entity::{async_graphql, note};
use graphql_core::note::Query;
use graphql_core::sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct NoteQuery;

#[Object]
impl NoteQuery {
    async fn get_notes(&self, ctx: &Context<'_>) -> Result<Vec<note::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Query::get_all_notes(conn)
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_note_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<note::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Query::find_note_by_id(conn, id)
            .await
            .map_err(|e| e.to_string())?)
    }
}