use actix_web::web;
use fancy_regex::Regex;

use crate::{
    crypto::hash::{create_hash, verify_hash},
    jwt::{handlers::create_auth_token, payload::AuthToken},
    models::user::User,
};

use super::{
    errors::{GetSelfError, LoginError, RegisterError},
    payloads::{LoginPayload, RegisterPayload},
    repository::UserRepository,
};

#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> UserService {
        UserService { repository }
    }

    pub async fn get_self(&self, id: i32) -> Result<User, GetSelfError> {
        match self.repository.get_user_by_id(id).await {
            Ok(user) => Ok(user),
            Err(sqlx::Error::RowNotFound) => Err(GetSelfError::UserNotFound),
            Err(err) => {
                log::error!("{err}");

                Err(GetSelfError::InternalServerError)
            }
        }
    }

    fn get_user_token_exp_seconds(&self, remember_me: &Option<bool>) -> i64 {
        let seven_days = chrono::Duration::days(7);
        let thirty_days = chrono::Duration::days(30);

        let now = chrono::offset::Local::now();

        if let Some(remember_me) = remember_me {
            if *remember_me {
                (now + thirty_days).timestamp()
            } else {
                (now + seven_days).timestamp()
            }
        } else {
            (now + seven_days).timestamp()
        }
    }

    pub async fn login(&self, body: web::Json<LoginPayload>) -> Result<String, LoginError> {
        let user = match self
            .repository
            .get_user_by_username_or_email(&body.username, &body.username)
            .await
        {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => return Err(LoginError::UserNotFound),
            Err(err) => {
                log::error!("{err}");

                return Err(LoginError::InternalServerError);
            }
        };

        if !verify_hash(&body.password, &user.password) {
            return Err(LoginError::IncorrectPassword);
        }

        let exp = self.get_user_token_exp_seconds(&body.remember_me);

        let payload = AuthToken::new(user.id as u64, user.username, user.name, exp);

        match create_auth_token(&payload) {
            Ok(token) => Ok(token),
            Err(err) => {
                log::error!("{err}");

                Err(LoginError::InternalServerError)
            }
        }
    }

    pub async fn register(
        &self,
        body: web::Json<RegisterPayload>,
    ) -> Result<String, RegisterError> {
        let _ = match self
            .repository
            .get_user_by_username_or_email(&body.username, &body.email)
            .await
        {
            Ok(_) => return Err(RegisterError::UserAlreadyExists),
            Err(sqlx::Error::RowNotFound) => (),
            Err(err) => {
                log::error!("{err}");

                return Err(RegisterError::InternalServerError);
            }
        };

        if body.username.trim().len() < 5 {
            return Err(RegisterError::BadRequest(
                "Username must be at least 5 characters long",
            ));
        }

        let username_regex = Regex::new(r"^[A-Za-z0-9._-]{5,}$").unwrap();
        if !username_regex.is_match(&body.username).unwrap() {
            return Err(RegisterError::BadRequest(
                "Username can only contain letters, numbers, dots, hyphens, and underscores",
            ));
        }

        let email_regex = Regex::new(r"^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,63})+$").unwrap();
        if !email_regex.is_match(&body.email).unwrap() {
            return Err(RegisterError::BadRequest("Invalid email address"));
        }

        if body.name.trim().len() == 0 {
            return Err(RegisterError::BadRequest("Name can't be empty"));
        }

        let password_length = body.password.trim().len();
        if password_length < 8 || password_length > 24 {
            return Err(RegisterError::BadRequest(
                "Password must be 8-24 characters long",
            ));
        }

        let password_regex =
            Regex::new(r"^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,24}$")
                .unwrap();
        if !password_regex.is_match(&body.password).unwrap() {
            return Err(RegisterError::BadRequest("Password must contain at least one upper and lowercase alphabets, one numeric character, and one special character"));
        }

        let hash = create_hash(&body.password);

        let user = match self
            .repository
            .insert_user(&body.username, &body.name, &body.email, &hash)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                log::error!("{err}");

                return Err(RegisterError::InternalServerError);
            }
        };

        let exp = self.get_user_token_exp_seconds(&body.remember_me);

        let payload = AuthToken::new(user.id as u64, user.username, user.name, exp);

        match create_auth_token(&payload) {
            Ok(token) => Ok(token),
            Err(err) => {
                log::error!("{err}");

                Err(RegisterError::InternalServerError)
            }
        }
    }
}
