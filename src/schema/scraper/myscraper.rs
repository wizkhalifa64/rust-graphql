use async_graphql::{Context, Object, Result, Upload};
use reqwest::Client;
use scraper::{Html, Selector};

use super::myscrapermodel::ScraperBody;
#[derive(Default)]
pub struct ScraperMutation;
#[Object]
impl ScraperMutation {
    async fn myscraper(
        &self,
        ctx: &Context<'_>,
        body: ScraperBody,
        file: Upload,
    ) -> Result<Vec<String>> {
        const USER_AGENT: &str =
            "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0";
        let upload_file = file.value(ctx).expect("Unable to read file");
        if let Some(file_type) = &upload_file.content_type {
            if file_type != "text/csv" {
                return Err("File type not supported".to_string().into());
            }
        };

        let scrat_ctx = Client::builder().user_agent(USER_AGENT).build().unwrap();
        let url = format!("https://en.wikipedia.org/wiki/mumbai_airport");
        let res = scrat_ctx.get(url).send().await.map_err(|err| err);
        let body: Result<Vec<String>, String> = match res {
            Ok(data) => {
                let new_body = data.text().await?;
                let html = Html::parse_fragment(&new_body);
                let selector = Selector::parse("span[class= 'mw-page-title-main' ]").unwrap();
                let scrap_text = html
                    .select(&selector)
                    .map(|x| x.text().next().unwrap().to_string())
                    .collect::<Vec<String>>();
                return Ok(scrap_text);
            }
            Err(_) => Err("Unable to scrap".to_string()),
        };

        Ok(body?)
    }
}
