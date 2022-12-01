use async_graphql::{Context, Object, Result};

use entity::async_graphql::{self, ErrorExtensions, InputObject, SimpleObject};
use entity::user;
use graphql_core::user::Mutation;
use migration::sea_orm::DatabaseConnection;

use crate::jwt::{create_access_token, validate_token, TokenType};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub email: String,
    pub scope_id: i32,
    pub first_name: String,
    pub last_name: String,
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
            first_name: self.first_name,
            last_name: self.last_name,
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
pub struct RefreshInput {
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
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let res = Mutation::create_user(conn, input.into_model_with_arbitrary_id()).await;

        match res {
            Ok(user) => Ok(user),
            Err(e) => Err(
                async_graphql::Error::new(e.to_string()).extend_with(|_, e| {
                    e.set("code", "INTERNAL_SERVER_ERROR");
                    e.set("details", "Something went wrong");
                }),
            ),
        }
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteUserResult> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        let res = Mutation::delete_user(conn, id).await;
        let res = res?;
        if res.rows_affected <= 1 {
            Ok(DeleteUserResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            Err(async_graphql::Error::new("Cannot delete user")
                .extend_with(|_, e| e.set("code", "INTERNAL_SERVER_ERROR")))
        }
    }
    pub async fn login_user(
        &self,
        ctx: &Context<'_>,
        input: LoginInput,
    ) -> Result<ValidLoginResult> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        let res = Mutation::login(conn, input.username, input.password).await;

        if res.is_ok() {
            let user = res.unwrap();
            let access = create_access_token(user.id, user.scope_id, TokenType::Access).await;
            let refresh = create_access_token(user.id, user.scope_id, TokenType::Refresh).await;

            Ok(ValidLoginResult { access, refresh })
        } else {
            Err(async_graphql::Error::new(res.err().unwrap().to_string())
                .extend_with(|_, e| e.set("code", "INVALID_LOGIN")))
        }
    }

    pub async fn refresh_user(
        &self,
        _ctx: &Context<'_>,
        input: RefreshInput,
    ) -> Result<ValidLoginResult> {
        let claim_token = validate_token(input.refresh.as_str()).await;
        if let Ok(claim) = claim_token {
            let access = create_access_token(claim.sub, claim.scope, TokenType::Access).await;
            let refresh = create_access_token(claim.sub, claim.scope, TokenType::Refresh).await;
            Ok(ValidLoginResult { access, refresh })
        } else {
            Err(async_graphql::Error::new("Invalid refresh token")
                .extend_with(|_, e| e.set("code", "INVALID_REFRESH_TOKEN")))
        }
    }
}
