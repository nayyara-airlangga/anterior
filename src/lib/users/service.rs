use actix_web::web;

use crate::{
    crypto::hash::verify_hash,
    jwt::{handlers::create_auth_token, payload::AuthToken},
    models::user::User,
};

use super::{errors::LoginError, payloads::LoginPayload, repository::UserRepository};

#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> UserService {
        UserService { repository }
    }

    pub async fn get_self(&self, id: i32) -> Result<User, sqlx::Error> {
        let user = self.repository.get_user_by_id(id).await?;

        Ok(user)
    }

    pub async fn login(&self, body: web::Json<LoginPayload>) -> Result<String, LoginError> {
        let user = match self
            .repository
            .get_user_by_username_or_email(&body.username)
            .await
        {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => return Err(LoginError::NotFound),
            Err(err) => {
                log::error!("{err}");

                return Err(LoginError::InternalServerError);
            }
        };

        if !verify_hash(&body.password, &user.password) {
            return Err(LoginError::IncorrectPassword);
        }

        let exp = if let Some(remember_me) = body.remember_me {
            if remember_me {
                (chrono::offset::Local::now() + chrono::Duration::days(30)).timestamp()
            } else {
                (chrono::offset::Local::now() + chrono::Duration::days(7)).timestamp()
            }
        } else {
            (chrono::offset::Local::now() + chrono::Duration::days(7)).timestamp()
        };

        let payload = AuthToken::new(user.id as u64, user.username, user.name, exp);

        match create_auth_token(&payload) {
            Ok(token) => Ok(token),
            Err(err) => {
                log::error!("{err}");

                Err(LoginError::InternalServerError)
            }
        }
    }
}
