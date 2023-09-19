use http_range_client::UreqHttpReader as HttpReader;
use polars::prelude::*;

/*
REMARK: Does require branch `polars` of http-range-client,
which is not released.
Polars requires MmapBytesReader::to_bytes or to_file, which doesn't make sense for SyncHttpRangeClient
*/

fn main() {
    env_logger::init();

    // let f = std::fs::File::open("../geoarrow-rs/fixtures/geoparquet/nybb.parquet").unwrap();
    // let mut reader = ReadLogger::new(f, Level::Debug, "READ");
    let mut reader = HttpReader::new(
        "https://github.com/kylebarron/geoarrow-rs/raw/main/fixtures/geoparquet/nybb.parquet",
    );
    let df = ParquetReader::new(&mut reader).finish().unwrap();
    println!("{df}");
}
