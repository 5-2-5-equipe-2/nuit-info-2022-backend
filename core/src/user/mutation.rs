use ::entity::prelude::User;
use ::entity::user;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, form_data: user::Model) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            username: Set(form_data.username.to_owned()),
            password: Set(form_data.password.to_owned()),
            email: Set(form_data.email.to_owned()),
            scope_id: Set(form_data.scope_id.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
            updated_at: Set(form_data.updated_at.to_owned()),
            ..Default::default()
        };
        println!("active_model: {:?}", active_model);
        let res = User::insert(active_model).exec(db).await?;

        Ok(user::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }
    //
    // pub async fn update_user_by_id(
    //     db: &DbConn,
    //     id: i32,
    //     form_data: user::Model,
    // ) -> Result<user::Model, DbErr> {
    //     let user: user::ActiveModel = user::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
    //         .map(Into::into)?;
    //
    //     user::ActiveModel {
    //         id: user.id,
    //         title: Set(form_data.title.to_owned()),
    //         text: Set(form_data.text.to_owned()),
    //     }
    //     .update(db)
    //     .await
    // }

    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }
}
