use polars::prelude::*;

fn no_panic(path: &str) -> PolarsResult<()> {
    let df = LazyFrame::scan_parquet(PlRefPath::new(path), Default::default())?
        .select([
            col("open_time").cast(DataType::Int64),
            col("vol").cast(DataType::Float64),
        ])
        .filter(
            col("open_time")
                .cast(DataType::Int64)
                .gt_eq(lit(1_577_836_800_000i64)),
        )
        .collect()?;

    println!("shape: {:?}", df.shape());
    Ok(())
}

fn would_panic(path: &str) -> PolarsResult<()> {
    let df = LazyFrame::scan_parquet(PlRefPath::new(path), Default::default())?
        .filter(
            col("open_time")
                .cast(DataType::Int64)
                .gt_eq(lit(1_577_836_800_000i64)),
        )
        .select([
            col("open_time").cast(DataType::Int64),
            col("vol").cast(DataType::Float64),
        ])
        .collect()?;

    println!("shape: {:?}", df.shape());
    Ok(())
}

fn main() -> PolarsResult<()> {
    // usage: cargo run -- <parquet-path>
    let path = std::env::args()
        .nth(1)
        .expect("usage: cargo run -- <parquet-path>");

    no_panic(&path)?;

    would_panic(&path)?;

    Ok(())
}
