use async_graphql::{Context, Object, Result, Upload};

#[derive(Default)]
pub struct FileMutation;
#[Object]
impl FileMutation {
    async fn single_upload(&self, ctx: &Context<'_>, file: Upload) -> Result<String> {
        let upload = file.value(ctx).unwrap();
        print!("{}", upload.filename);
        Ok(upload.filename)
    }
}
