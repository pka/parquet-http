use http_range_client::HttpReader;
use parquet::file::reader::{FileReader, Length, SerializedFileReader};
use std::fs::File;

/*
REMARK: Does require branch `parquet` of http-range-client,
which is not released, because Clone would be required.
*/

fn main() {
    env_logger::init();

    // let file = File::open("../geoarrow-rs/fixtures/geoparquet/nybb.parquet").unwrap();
    // let mut input = ReadLogger::new(file, Level::Debug, "READ");
    let mut input = HttpReader::new(
        "https://github.com/kylebarron/geoarrow-rs/raw/main/fixtures/geoparquet/nybb.parquet",
    );
    // Issue HEAD request for content-length
    let _ = input.get_content_length().unwrap();    
    let reader = SerializedFileReader::new(input).unwrap();

    let parquet_metadata = reader.metadata();
    assert_eq!(parquet_metadata.num_row_groups(), 1);

    let row_group_reader = reader.get_row_group(0).unwrap();
    assert_eq!(row_group_reader.num_columns(), 5);
}
