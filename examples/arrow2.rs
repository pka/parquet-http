use arrow2::error::Error;
use arrow2::io::parquet::read;
use http_range_client::UreqHttpReader as HttpReader;
use std::time::SystemTime;

fn main() -> Result<(), Error> {
    env_logger::init();

    // let f = std::fs::File::open("../geoarrow-rs/fixtures/geoparquet/nybb.parquet")?;
    // let mut reader = ReadLogger::new(f, Level::Debug, "READ");
    let mut reader = HttpReader::new(
        "https://github.com/kylebarron/geoarrow-rs/raw/main/fixtures/geoparquet/nybb.parquet",
    );

    // we can read its metadata:
    let metadata = read::read_metadata(&mut reader)?;

    // and infer a [`Schema`] from the `metadata`.
    let schema = read::infer_schema(&metadata)?;

    // we can filter the columns we need (here we select all)
    let schema = schema.filter(|_index, _field| true);

    // we can read the statistics of all parquet's row groups (here for each field)
    for field in &schema.fields {
        let statistics = read::statistics::deserialize(field, &metadata.row_groups)?;
        println!("{statistics:#?}");
    }

    // say we found that we only need to read the first two row groups, "0" and "1"
    let row_groups = metadata
        .row_groups
        .into_iter()
        .enumerate()
        .filter(|(index, _)| *index == 0 || *index == 1)
        .map(|(_, row_group)| row_group)
        .collect();

    // we can then read the row groups into chunks
    let chunks = read::FileReader::new(reader, row_groups, schema, Some(1024 * 8 * 8), None, None);

    let start = SystemTime::now();
    for maybe_chunk in chunks {
        let chunk = maybe_chunk?;
        assert!(!chunk.is_empty());
    }
    println!("took: {} ms", start.elapsed().unwrap().as_millis());
    Ok(())
}
