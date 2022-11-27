use async_graphql::{Context, Object, Result};

use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::user;
use graphql_core::user::Mutation;

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.
#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub email: String,
    pub scope_id: i32,
}

impl CreateUserInput {
    fn into_model_with_arbitrary_id(self) -> user::Model {
        user::Model {
            id: 0,
            username: self.username,
            password: self.password,
            email: self.email,
            created_at: chrono::Utc::now().to_string(),
            updated_at: chrono::Utc::now().to_string(),
            scope_id: self.scope_id,
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteUserResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(SimpleObject)]
pub struct ValidLoginResult {
    pub access: String,
    pub refresh: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(Mutation::create_user(conn, input.into_model_with_arbitrary_id()).await?)
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteUserResult> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        let res = Mutation::delete_user(conn, id)
            .await
            .expect("Cannot delete user");

        if res.rows_affected <= 1 {
            Ok(DeleteUserResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}
