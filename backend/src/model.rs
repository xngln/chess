use async_graphql::{MergedObject, Object};

pub mod auth_info;
pub mod user;

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[derive(Default)]
pub struct AuthInfoMutation;

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub UserQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(pub UserMutation, pub AuthInfoMutation);