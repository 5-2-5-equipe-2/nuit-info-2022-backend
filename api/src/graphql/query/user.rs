use async_graphql::{Context, Object, Result};

use entity::{async_graphql, user};
use graphql_core::user::Query;
use migration::sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Query::get_all_users(conn)
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Query::find_user_by_id(conn, id)
            .await
            .map_err(|e| e.to_string())?)
    }
}
