use ::entity::prelude::Scope;
use ::entity::scope;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_scope(db: &DbConn, form_data: scope::Model) -> Result<scope::Model, DbErr> {
        let active_model = scope::ActiveModel {
            title: Set(form_data.title.to_owned()),
            description: Set(form_data.description.to_owned()),
            ..Default::default()
        };
        let res = Scope::insert(active_model).exec(db).await?;

        Ok(scope::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }

    // pub async fn update_scope_by_id(
    //     db: &DbConn,
    //     id: i32,
    //     form_data: scope::Model,
    // ) -> Result<scope::Model, DbErr> {
    //     let scope: scope::ActiveModel = Scope::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find scope.".to_owned()))
    //         .map(Into::into)?;
    //
    //     scope::ActiveModel {
    //         id: scope.id,
    //         title: Set(form_data.title.to_owned()),
    //         text: Set(form_data.text.to_owned()),
    //     }
    //     .update(db)
    //     .await
    // }

    pub async fn delete_scope(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let scope: scope::ActiveModel = Scope::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find scope.".to_owned()))
            .map(Into::into)?;

        scope.delete(db).await
    }

    pub async fn delete_all_scopes(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Scope::delete_many().exec(db).await
    }
}
