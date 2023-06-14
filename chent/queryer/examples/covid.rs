use anyhow::{Ok, Result};
use polars::prelude::*;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://gitee.com/ekoclike/get/raw/main/qwe.csv";
    let data = reqwest::get(url).await?.text().await?;

    // 使用polars直接请求
    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;

    let filtered = df.filter(&df["new_deaths"].gt(500).unwrap())?;
    let arrow = [
        "location",
        "total_cases",
        "new_cases",
        "total_deaths",
        "new_deaths",
    ];
    let mut iter = arrow.into_iter();
    println!("{:?}", filtered.select(iter));
    Ok(())
}
