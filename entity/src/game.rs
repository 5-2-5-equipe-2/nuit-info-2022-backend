//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "game")]
#[graphql(concrete(name = "Game", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub health: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_user_id(user_id: i32) -> Select<Entity> {
        Self::find().filter(Column::UserId.eq(user_id))
    }
}
