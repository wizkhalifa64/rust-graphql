use super::filemodel::{FileBody, Fileresponse};
use async_graphql::{Context, Json, Object, Result, Upload};
use polars::prelude::*;
use serde_json::to_value;

#[derive(Default)]
pub struct FileMutation;
#[Object]
impl FileMutation {
    async fn single_upload(
        &self,
        ctx: &Context<'_>,
        file: Upload,
        body: FileBody,
    ) -> Result<Fileresponse> {
        let upload = file.value(ctx).unwrap();
        let file = upload.content;
        let df = CsvReader::new(file)
            .truncate_ragged_lines(true)
            .infer_schema(None)
            .has_header(true)
            .finish()?;
        let df_head = df.clone().head(Some(body.headcount));
        let df_describe = df.clone().describe(None)?;
        let response = Fileresponse {
            describe: Json(to_value(df_describe).unwrap()),
            head: Json(to_value(df_head).unwrap()),
        };
        Ok(response)
    }
}
