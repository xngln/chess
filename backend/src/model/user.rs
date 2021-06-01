use async_graphql::{Context, Object, SimpleObject, ID, Result};
use sqlx::postgres::PgPool;

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
    async fn signup(&self,  ctx: &Context<'_>, username: String, password: String) -> Result<i64> {
        // User signup
        let password_hash = "test password hash".to_string();
        let salt = "testsalt".to_string();

        let pool = ctx.data::<PgPool>().expect("Unable to access connection pool inside mutation resolver");

        create_user(&pool, username, password_hash, salt).await
    }
}

async fn create_user(pool: &PgPool, username: String, password_hash: String, salt: String) -> Result<i64> {
    let record = sqlx::query!(
        r#"
RETURNING id
        "#,
        
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}