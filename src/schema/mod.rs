use async_graphql::MergedObject;

use self::{auth::userschema::UserMutation, home::fileread::FileMutation};

pub mod auth;
pub mod home;

#[derive(Default, MergedObject)]
pub struct Mutation(UserMutation, FileMutation);
