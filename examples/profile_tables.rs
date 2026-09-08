//! One-input offline parser timing; use an external process profiler for peak RSS.
use sec_data_fetcher::extract_tables;
use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: profile_tables FILE")?;
    let content = std::fs::read_to_string(path)?;
    let start = Instant::now();
    let tables = extract_tables(&content);
    let elapsed = start.elapsed();
    println!(
        "{}",
        serde_json::json!({ "filingBytes": content.len(), "tables": tables.len(), "parseMs": elapsed.as_secs_f64() * 1000.0 })
    );
    std::hint::black_box(tables);
    Ok(())
}
