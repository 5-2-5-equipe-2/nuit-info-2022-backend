use ::entity::questions::Model;
use sea_orm::*;

use ::entity::prelude::Questions;

pub struct Query;

impl Query {
    pub async fn find_question_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        Questions::find_by_id(id).one(db).await
    }
}
