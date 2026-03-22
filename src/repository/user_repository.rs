use crate::model::user::User;

pub struct UserRepository {
    items: Vec<User>,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {
            items: vec![User { id: 1, username: "alpha".into(), email: "alpha@example.com".into(), active: true }],
        }
    }

    pub fn list(&self) -> Vec<User> {
        self.items.clone()
    }

    pub fn create(&mut self, username: String, email: String) -> User {
        let id = (self.items.len() as u64) + 1;
        let user = User { id, username, email, active: true };
        self.items.push(user.clone());
        user
    }
}