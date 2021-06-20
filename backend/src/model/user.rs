use async_graphql::{Context, ErrorExtensions, Object, SimpleObject, ID, Result};
use crate::auth;
use crate::error::Error;
use sqlx::postgres::PgPool;

#[derive(Clone, SimpleObject)]
pub struct User {
    id: ID, 
    username: String,
    elo: u16,
    wins: u32,
    losses: u32,
    draws: u32,
}

#[Object]
impl super::UserQuery {
    async fn user(&self, username: String) -> Result<Option<User>> {
        // Get a user by username from db
        let user = User {
            id: ID(username),
            username: "firstuser".to_string(),
            elo: 3000,
            wins: 10,
            losses: 7,
            draws: 9,
        };

        Ok(Some(user))
    }
}

#[Object]
impl super::UserMutation {
    async fn signup(&self,  ctx: &Context<'_>, username: String, password: String) -> Result<String> {
        // User signup
        let password_hash = auth::hash_password(password);

        let pool = ctx.data::<PgPool>().expect("Unable to access connection pool inside signup resolver");

        create_user(&pool, username, password_hash).await
    }

    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<String> {
        // User Login
        let pool = ctx.data::<PgPool>().expect("Unable to access connection inside login resolver");

        match get_user_by_username(&pool, username).await {
            Ok(user) => {
				// !!!!!!!!!!!!!!!!!!!!!! fix
                if !auth::verify_password(password, user.username).expect("Could not verify password") {
                    // wrong password
                    return Err(Error::WrongCredentials.extend());
                }

                let jwt = auth::create_jwt(&user.id.to_string(), &auth::Role::User).expect("Failed to create jwt");
                Ok(jwt)
            },
            // unable to get user by username
            Err(e) => Err(Error::UserNotFound.extend()),
        }
    }
}

async fn create_user(pool: &PgPool, username: String, password_hash: String) -> Result<String> {
    let init_elo = 1200i32;
    let init_wins = 0i32;
    let init_losses = 0i32;
    let init_draws = 0i32;

	// TODO: move query to db file
    let record = sqlx::query!(
        r#"
INSERT INTO users ( username, elo, wins, losses, draws, password_hash, salt)
VALUES ( $1, $2, $3, $4, $5, $6, $7 )
RETURNING id
        "#,
        username, init_elo, init_wins, init_losses, init_draws, password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id.to_string())
}

async fn get_user_by_username(pool: &PgPool, username: String) -> Result<User> {
	// TODO: move query object into db file, use query_as! instead of query -> https://github.com/launchbadge/sqlx#compile-time-verification 
    let record = sqlx::query!(
        r#"
SELECT *
FROM users
WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await?;

    let user = User {
        id: ID::from(record.id.to_string()),
        username: match record.username {
            Some(name) => name,
            None       => panic!("User record retrieved from the database has no username") 
        },
        elo: match record.elo {
            Some(x) => x as u16,
            None    => 1200,
        },
        wins: match record.wins {
            Some(x) => x as u32,
            None    => 0,
        },
        losses: match record.losses {
            Some(x) => x as u32,
            None    => 0,
        },
        draws: match record.draws {
            Some(x) => x as u32,
            None    => 0,
        },
    };

    Ok(user)
}