use async_graphql::MergedObject;

use self::{home::fileread::FileMutation, scraper::myscraper::ScraperMutation};

pub mod home;
pub mod scraper;
#[derive(Default, MergedObject)]
pub struct Mutation(FileMutation, ScraperMutation);
