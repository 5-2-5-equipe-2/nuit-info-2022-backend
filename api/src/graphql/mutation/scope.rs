use async_graphql::{Context, Object, Result};

use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::scope;
use graphql_core::scope::Mutation;
use migration::sea_orm::DatabaseConnection;

use crate::db::Database;

#[derive(InputObject)]
pub struct CreateScopeInput {
    pub title: String,
    pub description: String,
}

impl CreateScopeInput {
    fn into_model_with_arbitrary_id(self) -> scope::Model {
        scope::Model {
            id: 0,
            title: self.title,
            description: self.description,
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteScopeResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct ScopeMutation;

#[Object]
impl ScopeMutation {
    pub async fn create_scope(
        &self,
        ctx: &Context<'_>,
        input: CreateScopeInput,
    ) -> Result<scope::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Ok(Mutation::create_scope(conn, input.into_model_with_arbitrary_id()).await?)
    }

    pub async fn delete_scope(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteScopeResult> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        let res = Mutation::delete_scope(conn, id)
            .await
            .expect("Cannot delete scope");

        if res.rows_affected <= 1 {
            Ok(DeleteScopeResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}
