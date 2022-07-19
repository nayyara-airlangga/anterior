use crate::models::user::User;

use super::repository::UserRepository;

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
}
