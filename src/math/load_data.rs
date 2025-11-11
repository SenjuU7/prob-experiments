use polars::prelude::*;


pub fn example() -> PolarsResult<DataFrame> {
    let data = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(("iris.csv").into()))
    .unwrap()
    .finish()
    .unwrap();


    println!("{:?}",data);

    Ok(data)
}