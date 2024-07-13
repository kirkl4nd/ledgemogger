use uuid::Uuid;

pub struct UserInput {
    pub username: String,
    pub name: String,
}

/// Represents user info from a client
impl UserInput {
    pub fn new(username: String, name: String) -> Self {
        Self { username, name }
    }

    /// Converts the user input to a user with an id
    pub fn to_user(self) -> User {
        User {
            id: Uuid::new_v4(),
            username: self.username,
            name: self.name,
        }
    }
}

/// Represents a user in the system
#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: String,
}

impl User {
    pub fn new(username: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            name,
        }
    }
}