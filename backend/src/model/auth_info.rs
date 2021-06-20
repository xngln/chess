use async_graphql::{Context, ErrorExtensions, Object, SimpleObject, ID, Result};
use crate::auth;
use crate::error::Error;
use sqlx::postgres::PgPool;

#[derive(Clone, SimpleObject)]
pub struct AuthInfo {
    user_id: ID,
    password_hash: String,
    refresh_token: String,
}

#[Object]
impl super::AuthInfoMutation {
    async fn refreshtoken(&self, username: String) -> Result<Option<String>> {
		// genereate another jwt using provided refresh token
        Ok(Some(String::from("sample-jwt_token")))
    }
}

pub fn create_refresh_token() {

}