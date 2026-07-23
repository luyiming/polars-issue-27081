# Reproduce polars

```sh
cargo run -- foo.parquet
```

output:
```
shape: (72052, 2)

thread 'async-executor-7' (3741684) panicked at /Users/luyiming/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/polars-stream-0.54.4/src/nodes/io_sources/parquet/row_group_decode.rs:568:9:
assertion `left == right` failed
  left: 72053
 right: 72052
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'async-executor-0' (3741677) panicked at /Users/luyiming/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/polars-stream-0.54.4/src/nodes/io_sources/parquet/mod.rs:360:74:
called `Result::unwrap()` on an `Err` value: JoinError::Panic(Id(23), "assertion `left == right` failed\n  left: 72053\n right: 72052", ...)
```