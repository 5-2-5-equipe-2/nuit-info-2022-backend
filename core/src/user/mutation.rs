extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::*;

use ::entity::prelude::User;
use ::entity::{questions, user};

use thiserror::Error;

pub struct Mutation;

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Password too weak")]
    PasswordTooWeak,
    #[error("Invalid email")]
    InvalidEmail,
    #[error(transparent)]
    DBError(#[from] sea_orm::error::DbErr),
}

impl Mutation {}

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: user::Model,
    ) -> Result<user::Model, CreateUserError> {
        // check if user already exists
        let user = user::Entity::find_by_username(&form_data.username);
        if user.count(db).await? > 0 {
            return Err(CreateUserError::UserAlreadyExists);
        }
        let user = user::Entity::find_by_email(&form_data.email);
        if user.count(db).await? > 0 {
            return Err(CreateUserError::UserAlreadyExists);
        }

        // check if password is valid
        if form_data.password.len() < 8 {
            return Err(CreateUserError::PasswordTooWeak);
        }

        // check if email is valid
        if !form_data.email.contains('@') {
            return Err(CreateUserError::InvalidEmail);
        }

        let active_model = user::ActiveModel {
            id: Default::default(),
            username: Set(form_data.username.to_owned()),
            password: Set(hash(&form_data.password, 4).unwrap()),
            email: Set(form_data.email.to_owned()),
            scope_id: Set(form_data.scope_id.to_owned()),
            first_name: Set(form_data.first_name.to_owned()),
            last_name: Set(form_data.last_name.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
            updated_at: Set(form_data.updated_at.to_owned()),
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
    pub async fn login(
        db: &DbConn,
        username: String,
        password: String,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_username(&username)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;
        let password_hash = user.password.unwrap();
        let is_valid = verify(password, &password_hash).unwrap();
        if is_valid {
            Ok(user::Model {
                id: user.id.unwrap(),
                username: user.username.unwrap(),
                password: "".to_string(),
                email: user.email.unwrap(),
                created_at: user.created_at.unwrap(),
                updated_at: user.updated_at.unwrap(),
                scope_id: user.scope_id.unwrap(),
                first_name: user.first_name.unwrap(),
                last_name: user.last_name.unwrap(),
            })
        } else {
            Err(DbErr::Custom("Invalid password.".to_owned()))
        }
    }
}
