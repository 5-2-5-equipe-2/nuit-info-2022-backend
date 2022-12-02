use ::entity::game::Model;
use sea_orm::*;

use ::entity::game;
use ::entity::prelude::Game;

pub struct Query;

impl Query {
    pub async fn find_game_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        Game::find_by_id(id).one(db).await
    }

    pub async fn find_game_by_user_id(db: &DbConn, user_id: i32) -> Result<Option<Model>, DbErr> {
        Game::find_by_user_id(user_id).one(db).await
    }
}
