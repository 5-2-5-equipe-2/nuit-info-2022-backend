use crate::jwt::{validate_token, TokenType};

use async_graphql::{Context, Object, Result};

use entity::async_graphql::{self, ErrorExtensions, InputObject, SimpleObject};
use entity::game;
use graphql_core::game::Mutation;
use graphql_core::sea_orm::DatabaseConnection;

#[derive(InputObject)]
pub struct StartGameInput {
    pub token: String,
}

impl StartGameInput {
    async fn into_model_with_arbitrary_id(self) -> game::Model {
        let user = validate_token(&self.token).await.unwrap();
        game::Model {
            id: 0,
            user_id: user.sub,
            health: 100,
        }
    }
}

#[derive(SimpleObject)]
pub struct AddGameResult {
    pub success: bool,
}

#[derive(InputObject)]
pub struct AnswerQuestionInput {
    pub token: String,
    pub question_id: i32,
    pub answer: String,
}

#[derive(SimpleObject)]
pub struct AnswerQuestionResult {
    pub success: bool,
}

#[derive(Default)]
pub struct GameMutation;

#[Object]
impl GameMutation {
    pub async fn start_game(
        &self,
        ctx: &Context<'_>,
        input: StartGameInput,
    ) -> Result<game::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let res = Mutation::start_game(conn, input.into_model_with_arbitrary_id().await).await;

        match res {
            Ok(game) => Ok(game),
            Err(e) => Err(
                async_graphql::Error::new(e.to_string()).extend_with(|_, e| {
                    e.set("code", "INTERNAL_SERVER_ERROR");
                    e.set("details", "Something went wrong");
                }),
            ),
        }
    }

    pub async fn answer_question(
        &self,
        ctx: &Context<'_>,
        input: AnswerQuestionInput,
    ) -> Result<AnswerQuestionResult> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let user_id = validate_token(&input.token).await.unwrap();
        let res =
            Mutation::answer_question(conn, user_id.sub, input.question_id, input.answer).await;

        match res {
            Ok(game) => Ok(AnswerQuestionResult { success: true }),
            Err(e) => Err(
                async_graphql::Error::new(e.to_string()).extend_with(|_, e| {
                    e.set("code", "INTERNAL_SERVER_ERROR");
                    e.set("details", "Something went wrong");
                }),
            ),
        }
    }
}
