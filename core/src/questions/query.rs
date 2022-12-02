use ::entity::questions::Model;
use sea_orm::*;

use ::entity::prelude::Questions;

use rand::prelude::*;

pub struct Query;

impl Query {
    pub async fn find_question_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        Questions::find_by_id(id).one(db).await
    }

    pub async fn find_random_question(db: &DbConn) -> Result<Option<Model>, DbErr> {
        Questions::find()
            .all(db)
            .await
            .map(|questions| questions.choose(&mut rand::thread_rng()).map(|q| q.clone()))
    }
}
