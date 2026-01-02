use std::fs::File;
use polars_core::prelude::*;
use polars_io::prelude::*;

pub fn read_from_path(path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(path)?
            .has_header(true)
            .finish()
}