use crate::model::user::{User, UserCreate};
use crate::repository::user_repository::UserRepository;

pub struct UserService {
    repo: UserRepository,
}

impl Default for UserService {
    fn default() -> Self {
        UserService { repo: UserRepository::new() }
    }
}

impl UserService {
    pub fn list_users(&self) -> Vec<User> {
        self.repo.list()
    }

    pub fn create_user(&mut self, payload: UserCreate) -> User {
        self.repo.create(payload.username, payload.email)
    }
}