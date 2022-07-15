use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: Uuid,
}

impl User {
    pub fn new(user_id: Uuid) -> User {
        User { user_id }
    }
}
