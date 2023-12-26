use super::myscrapermodel::ScraperBody;
use async_graphql::{Context, Object, Result, Upload};
use polars::prelude::*;
use reqwest::Client;
use scraper::{Html, Selector};
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
        let data_frame = CsvReader::new(upload_file.content)
            .truncate_ragged_lines(true)
            .infer_schema(Some(100))
            .has_header(true)
            .finish()?;
        let selected_df = data_frame.clone().select(["city"]).map(|s| {
            let city = s.column("city").unwrap();
            let city_list: Vec<_> = city.utf8().into_iter().collect();
            println!("{:?}", city_list.get(0));
        });

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
