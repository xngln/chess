use async_graphql::{Object, SimpleObject, ID, Result};

#[derive(Clone, SimpleObject)]
pub struct User {
    id: ID, 
    username: String,
    elo: u16,
    wins: u32,
    losses: u32,
    draws: u32,
    password_hash: String,
    salt: String,
}

#[Object]
impl super::QueryRoot {
    async fn user(&self, username: String) -> Result<Option<User>> {
        // Get a user by username from db
        let user = User {
            id: ID(username),
            username: "firstuser".to_string(),
            elo: 3000,
            wins: 10,
            losses: 7,
            draws: 9,
            password_hash: "thepasswordhash".to_string(),
            salt: "saltyboi".to_string(),
        };

        Ok(Some(user))
    }
}

#[Object]
impl super::MutationRoot {
    async fn signup(&self,  username: String, password: String) -> Result<bool> {
        // User signup

        Ok(true)
    }
}