use async_graphql::{Context, Object, Result};

use entity::async_graphql::{self, ErrorExtensions, InputObject, SimpleObject};
use entity::questions;
use graphql_core::questions::Mutation;
use migration::sea_orm::DatabaseConnection;

use crate::jwt::{create_access_token, validate_token, TokenType};

#[derive(InputObject)]
pub struct AddQuestionInput {
    pub question: String,
    pub answer: String,
    pub category: String,
    pub a1: String,
    pub a2: String,
}

impl AddQuestionInput {
    fn into_model_with_arbitrary_id(self) -> questions::Model {
        questions::Model {
            id: 0,
            question: self.question,
            answer: self.answer,
            category: self.category,
            a1: self.a1,
            a2: self.a2,
        }
    }
}

#[derive(SimpleObject)]
pub struct AddQuestionResult {
    pub success: bool,
}

#[derive(Default)]
pub struct QuestionsMutation;

#[Object]
impl QuestionsMutation {
    pub async fn add_question(
        &self,
        ctx: &Context<'_>,
        input: AddQuestionInput,
    ) -> Result<questions::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let res = Mutation::add_question(conn, input.into_model_with_arbitrary_id()).await;

        match res {
            Ok(question) => Ok(question),
            Err(e) => Err(
                async_graphql::Error::new(e.to_string()).extend_with(|_, e| {
                    e.set("code", "INTERNAL_SERVER_ERROR");
                    e.set("details", "Something went wrong");
                }),
            ),
        }
    }
}
