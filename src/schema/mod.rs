use async_graphql::MergedObject;

use self::home::fileread::FileMutation;

// pub mod auth;
pub mod home;

#[derive(Default, MergedObject)]
pub struct Mutation(FileMutation);
