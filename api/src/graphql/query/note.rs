use async_graphql::{Context, Object, Result};

use entity::{async_graphql, note};
use graphql_core::note::Query;
use graphql_core::sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct NoteQuery;

#[Object]
impl NoteQuery {
    async fn get_notes(&self, ctx: &Context<'_>) -> Result<String> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok("HONEY POT".to_string())
    }

    async fn get_note_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<String>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Some("HONEY POT".to_string()))
    }
}
