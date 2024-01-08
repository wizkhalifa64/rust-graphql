use crate::AppState;

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
    ) -> Result<String> {
        const USER_AGENT: &str =
            "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0";
        let scrat_ctx = Client::builder().user_agent(USER_AGENT).build().unwrap();
        // let upload_file = file.value(ctx).expect("Unable to read file");
        // if let Some(file_type) = &upload_file.content_type {
        //     if file_type != "text/csv" {
        //         return Err("File type not supported".to_string().into());
        //     }
        // };

        // let data_frame = CsvReader::new(upload_file.content)
        //     .truncate_ragged_lines(true)
        //     .infer_schema(Some(100))
        //     .has_header(true)
        //     .finish()?;

        // let data_series = data_frame.column("city").expect("l");
        // let data_list = data_series.utf8()?;
        // for item in data_list {
        //     let city_list = item.map(str::to_string).unwrap();
        //     let url = format!("https://en.wikipedia.org/wiki/{}", city_list);
        //     let res = scrat_ctx.get(url).send().await.map_err(|err| err);
        //     let body: Result<String, String> = match res {
        //         Ok(data) => {
        //             let new_body = data.text().await?;
        //             let html = Html::parse_fragment(&new_body);
        //             let selector = Selector::parse(["span[class= 'mw-page-title-main' ]","table[class= 'infobox vcard' ]"]).unwrap();
        //             let scrap_text = html
        //                 .select(&selector)
        //                 .map(|x| x.text().next().unwrap().to_string())
        //                 .collect::<String>();
        //             return Ok(scrap_text);
        //         }
        //         Err(_) => Err("Unable to scrap".to_string()),
        //     };

        //     println!("{:?}", body);
        // }
        let url = format!("https://en.wikipedia.org/wiki/{}", body.airport);
        let res = scrat_ctx.get(url).send().await.map_err(|err| err);
        let body: Result<String, String> = match res {
            Ok(data) => {
                let new_body = data.text().await?;
                let html = Html::parse_fragment(&new_body);
                let db = &ctx.data::<AppState>()?.db;
                let selector_header =
                    Selector::parse("table[class= 'infobox vcard' ]  tbody  div").unwrap();
                let selector_th =
                    // Selector::parse("table[class= 'infobox vcard' ]  tbody  div[class= 'hlist' ]")
                    Selector::parse("table[class= 'infobox vcard' ]  th")
                        .unwrap();
                let selector_td = Selector::parse("table[class= 'infobox vcard' ]  td").unwrap();
                let airportHeader = html
                    .select(&selector_header)
                    .map(|elm| match elm.text().next() {
                        Some(val) => Some(val.to_string()),
                        None => None,
                    })
                    .collect::<Option<String>>();
                // let new_airport = CreateAirportDetails {};
                let scrap_text: Option<String> = html
                    .select(&selector_td)
                    .map(|x| match x.text().next() {
                        Some(val) => Some(val.to_string()),
                        None => None,
                    })
                    .collect::<Option<String>>();
                println!("{:?}", scrap_text);
                return Ok("Completed".to_string());
            }
            Err(_) => Err("Unable to scrap".to_string()),
        };
        Ok(body?)
    }
}
