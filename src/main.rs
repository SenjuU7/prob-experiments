use polars::error::PolarsResult;

mod math;
mod utils;

fn main() -> PolarsResult<()>{
    math::load_data::example();


    Ok(())
}
