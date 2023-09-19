use datafusion::prelude::*;
use object_store::http::HttpBuilder;
use std::sync::Arc;
use url::Url;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    env_logger::init();

    let ctx = SessionContext::new();
    let base_url = Url::parse("https://github.com").unwrap();
    let http_store = HttpBuilder::new()
        .with_url(base_url.clone())
        .build()
        .unwrap();
    ctx.runtime_env()
        .register_object_store(&base_url, Arc::new(http_store));
    ctx.register_parquet(
        "nybb",
        "https://github.com/kylebarron/geoarrow-rs/raw/main/fixtures/geoparquet/nybb.parquet",
        ParquetReadOptions::default(),
    )
    .await?;

    let df = ctx
        .sql(r#"SELECT "BoroCode", "BoroName" FROM nybb"#)
        .await?;

    df.show().await?;
    Ok(())
}
