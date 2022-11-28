use async_graphql::*;
use sea_orm::entity::prelude::*;
use sea_orm::DeleteMany;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(concrete(name = "User", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    pub scope_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::scope::Entity",
        from = "Column::ScopeId",
        to = "super::scope::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Scope,
}

impl Related<super::scope::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scope.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        // do not send password field
        Self::find().filter(Column::Id.eq(id))
    }
    pub fn find_by_username(username: &str) -> Select<Entity> {
        Self::find().filter(Column::Username.eq(username))
    }
    pub fn find_by_email(email: &str) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
