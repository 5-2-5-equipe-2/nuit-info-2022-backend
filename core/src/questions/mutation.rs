use sea_orm::sea_query::ColumnSpec::Default;
use sea_orm::*;

use ::entity::prelude::Questions;
use ::entity::prelude::User;
use ::entity::questions;

use thiserror::Error;

pub struct Mutation;

#[derive(Error, Debug)]
pub enum AddQuestionError {
    #[error("Question already exists")]
    QuestionAlreadyExists,
    #[error(transparent)]
    DBError(#[from] sea_orm::error::DbErr),
}

impl Mutation {}

impl Mutation {
    pub async fn add_question(
        db: &DbConn,
        form_data: questions::Model,
    ) -> Result<questions::Model, AddQuestionError> {
        let current_question = questions::Entity::find_by_question(&form_data.question);
        if current_question.count(db).await.unwrap() > 0 {
            return Err(AddQuestionError::QuestionAlreadyExists);
        }

        let active_model = questions::ActiveModel {
            id: NotSet,
            question: Set(form_data.question.to_owned()),
            answer: Set(form_data.answer.to_owned()),
            a1: Set(form_data.a1.to_owned()),
            a2: Set(form_data.a2.to_owned()),
            category: Set(form_data.category.to_owned()),
            explanation: Set(form_data.explanation.to_owned()),
        };

        let res = Questions::insert(active_model).exec(db).await?;
        Ok(questions::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }
}
