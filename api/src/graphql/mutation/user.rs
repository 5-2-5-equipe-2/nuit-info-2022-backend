use async_graphql::{Context, Object, Result};

use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::user;
use graphql_core::user::Mutation;

use crate::db::Database;
use crate::jwt::{create_access_token, TokenType};

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
            password: Some(self.password),
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

#[derive(InputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
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
    pub async fn login_user(
        &self,
        ctx: &Context<'_>,
        input: LoginInput,
    ) -> Result<ValidLoginResult> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        let res = Mutation::login(conn, input.username, input.password)
            .await
            .expect("Cannot login");

        // jwt
        let access = create_access_token(res.id, res.scope_id, TokenType::Access).await;
        let refresh = create_access_token(res.id, res.scope_id, TokenType::Refresh).await;

        Ok(ValidLoginResult { access, refresh })
    }
}
