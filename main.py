import polars as pl


def main():
    df = pl.scan_parquet("foo.parquet").filter(pl.col("open_time").cast(pl.Int64) >= 1_577_836_800_000).select(
        [
            pl.col("open_time").cast(pl.Int64),
            pl.col("vol").cast(pl.Float64),
        ]
    ).collect()

    print(df.shape)

if __name__ == "__main__":
    main()
