use ::entity::async_graphql::futures_util::AsyncWriteExt;
use sea_orm::sea_query::ColumnSpec::Default;
use sea_orm::*;

use ::entity::game;
use ::entity::prelude::Game;
use ::entity::prelude::User;
use ::entity::questions;

use thiserror::Error;

pub struct Mutation;

#[derive(Error, Debug)]
pub enum AddGameError {
    #[error("Game already exists")]
    GameAlreadyExists,
    #[error(transparent)]
    DBError(#[from] sea_orm::error::DbErr),
}

#[derive(Error, Debug)]
pub enum AnswerQuestionError {
    #[error("Game does not exist")]
    GameDoesNotExist,
    #[error("Question does not exist")]
    QuestionDoesNotExist,
    #[error(transparent)]
    DBError(#[from] sea_orm::error::DbErr),
}

impl Mutation {}

impl Mutation {
    pub async fn start_game(
        db: &DbConn,
        form_data: game::Model,
    ) -> Result<game::Model, AddGameError> {
        let current_game = Game::find_by_user_id(form_data.user_id.clone());
        if current_game.count(db).await.unwrap() > 0 {
            return Err(AddGameError::GameAlreadyExists);
        }

        let active_model = game::ActiveModel {
            user_id: Set(form_data.user_id.clone()),
            health: Set(form_data.health.clone()),
            ..core::default::Default::default()
        };

        let res = Game::insert(active_model).exec(db).await?;
        Ok(game::Model {
            id: res.last_insert_id,
            user_id: form_data.user_id,
            health: form_data.health,
        })
    }

    pub async fn answer_question(
        db: &DbConn,
        user_id: i32,
        question_id: i32,
        answer: String,
    ) -> Result<game::Model, AnswerQuestionError> {
        let current_game = Game::find_by_user_id(user_id).one(db).await;
        if let Err(e) = current_game {
            return Err(AnswerQuestionError::GameDoesNotExist);
        }
        let current_game = current_game.unwrap().unwrap();

        let question = questions::Entity::find_by_id(question_id).one(db).await;
        if let Err(e) = question {
            return Err(AnswerQuestionError::QuestionDoesNotExist);
        }
        let question = question.unwrap().unwrap();

        if answer == question.answer {
            let active_model = game::ActiveModel {
                health: Set(current_game.health + 1),
                ..core::default::Default::default()
            };

            let res = Game::update(active_model)
                .filter(game::Column::UserId.eq(current_game.user_id))
                .exec(db)
                .await?;

            return Ok(game::Model {
                id: current_game.id,
                user_id: current_game.user_id,
                health: current_game.health + 1,
            });
        }
        Ok(current_game)
    }

    pub async fn end_game(db: &DbConn, user_id: i32) -> Result<(), AnswerQuestionError> {
        let current_game = Game::find_by_user_id(user_id).one(db).await;
        if let Err(e) = current_game {
            return Err(AnswerQuestionError::GameDoesNotExist);
        }
        let current_game = current_game.unwrap().unwrap();

        Game::delete()
            .filter(game::Column::UserId.eq(current_game.user_id))
            .exec(db)
            .await?;

        Ok(())
    }
}
