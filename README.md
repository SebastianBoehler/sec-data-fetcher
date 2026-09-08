<p align="center">
  <img src="https://raw.githubusercontent.com/SebastianBoehler/sec-data-fetcher/main/docs/assets/banner.svg" alt="SEC Data Fetcher — EDGAR filings and financial data for Rust" width="100%" />
</p>

<p align="center">
  <a href="https://github.com/SebastianBoehler/sec-data-fetcher/actions/workflows/test.yml"><img src="https://github.com/SebastianBoehler/sec-data-fetcher/actions/workflows/test.yml/badge.svg" alt="CI" /></a>
  <a href="https://github.com/SebastianBoehler/sec-data-fetcher/releases"><img src="https://img.shields.io/github/v/release/SebastianBoehler/sec-data-fetcher" alt="Release" /></a>
  <img src="https://img.shields.io/badge/Rust-1.88%2B-orange" alt="Rust 1.88 or newer" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT license" /></a>
</p>

An async **Rust library and CLI for SEC EDGAR filings and financial data**. Look up companies, retrieve submissions and standardized XBRL facts, download individual filings, extract HTML tables and parse XML. Built for research scripts and financial-data pipelines, with a native executable and no Node.js runtime.

Public SEC endpoints need no API key. Identify your application with a real contact email. Version 3 replaces the previous TypeScript implementation; the old npm package remains at version 2 and does not contain the Rust implementation.

## Install

Requires Rust 1.88+ and a native C/C++ build toolchain for the TLS dependency. Install from the repository:

```sh
cargo install --git https://github.com/SebastianBoehler/sec-data-fetcher --locked
sec-data-fetcher --help
```

For reproducible installations, add `--rev <commit>` using a tested release commit. This project is not yet published on crates.io.

## CLI

```sh
export SEC_USER_AGENT='YourApp you@your-domain.com'
sec-data-fetcher lookup AAPL
sec-data-fetcher submissions 320193 > submissions.json
sec-data-fetcher facts 320193 > facts.json
sec-data-fetcher reports 320193 --after 2026-01-01 --forms 10-K,10-Q
```

`reports` returns metadata only. The date comparison is strictly **after**, and amendments must be requested explicitly (for example `10-K/A`). It reads `filings.recent`; historical shards in `filings.files` are exposed as metadata but are not traversed automatically.

Download a document using its public SEC archive URL, or parse files already on disk:

```sh
sec-data-fetcher fetch "$SEC_FILING_URL" > filing.html
sec-data-fetcher tables --file filing.html > tables.json
sec-data-fetcher xml --file filing.xml > document.json
```

`tables` and `xml` also accept `--url`. Local parsing needs no User-Agent and makes no requests. Structured results go to stdout as JSON; errors go to stderr with a nonzero exit status. `lookup` returns a padded CIK string or JSON `null` when no ticker matches.

## Rust library

Add the repository dependency and Tokio to your application:

```toml
[dependencies]
sec-data-fetcher = { git = "https://github.com/SebastianBoehler/sec-data-fetcher" }
tokio = { version = "1", features = ["macros", "rt"] }
```

Pin a `rev` for reproducible builds. This example selects one report before downloading:

```rust
use sec_data_fetcher::{SecClient, NaiveDate, extract_tables};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SecClient::with_rate_limit(std::env::var("SEC_USER_AGENT")?, 2)?;
    let cik = client.cik_lookup("AAPL").await?.ok_or("Ticker not found")?;
    let reports = client.get_reports(
        &cik, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(), &["10-K", "10-Q"],
    ).await?;
    let report = reports.first().ok_or("No matching report")?;
    let html = client.fetch_filing(&report.url()?).await?;
    println!("{} tables", extract_tables(&html).len());
    Ok(())
}
```

| API | Result |
| --- | --- |
| `cik_lookup(ticker)` | Optional validated, zero-padded `Cik` |
| `get_company_data(&cik)` | Typed submissions and recent filing columns |
| `get_company_facts(&cik)` | Typed facts grouped by taxonomy, concept and unit |
| `get_reports(&cik, after, forms)` | Selected filing metadata with validated `url()` |
| `fetch_filing(url)` | One document's text |
| `extract_tables(html)` | `Vec<Vec<Vec<String>>>` of tables, rows and cells |
| `parse_xml(xml)` | Ordered `XmlElement` tree with namespace URIs |
| `extract_tables_from_filing_url(url)` | Download and extract tables |
| `get_object_from_url(url)` | Download and parse XML |

Async methods return `Result`; parsing XML can fail, while HTML follows HTML5 error recovery. Run `cargo doc --open` for local API documentation.

## Data semantics and limits

- Company facts preserve units, dates and accession provenance. They are **not a deduplicated time series**: periods can recur across filings and amendments. Integer JSON values retain 64-bit precision; decimal JSON values use floating point.
- Tables preserve cell text and separate nested tables. Parent cells retain nested text. Rowspan/colspan and financial-statement semantics are not reconstructed.
- XML retains text (including leading zeros), namespace URIs and mixed-content order. Prefix spelling, comments and processing instructions are omitted. DTDs are rejected and element depth is limited to 128. This is not an iXBRL or SGML statement parser.
- Documents and parsed output still reside in memory. Download and process one document at a time for bounded workloads; this is not a streaming archive ingester.

## SEC access

Use a shared `SecClient` or its clones: they share connections and a paced request budget. Default is 10 requests/second; `with_rate_limit` or CLI `--requests-per-second` accepts 1–10. The SEC's limit applies **per user across all machines**, so coordinate separate clients and processes yourself.

Requests time out after 30 seconds. There are no automatic retries; redirects and other non-success responses surface with their HTTP status. Document URLs must use HTTPS on `www.sec.gov` or `data.sec.gov`, without credentials or custom ports. `403` can reflect network or identification restrictions; `429` calls for a lower request rate.

Rust can reduce local parsing overhead and process memory. It does not accelerate the SEC's publication schedule or bypass its access limits. See [SEC fair-access guidance](https://www.sec.gov/about/developer-resources) and [XBRL API coverage](https://www.sec.gov/search-filings/edgar-application-programming-interfaces).

## Develop and contribute

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
SEC_USER_AGENT='YourApp you@your-domain.com' cargo run --locked --release --example live_smoke
```

Deterministic tests use local fixtures and a loopback HTTP server. The separate live smoke makes four SEC requests: ticker mapping, submissions, facts and one filing. CI checks supported toolchains and operating systems; a weekly live workflow checks upstream access and compatibility.

See [contributing](CONTRIBUTING.md), [architecture](docs/architecture.md), [migration](docs/migration-v3.md), [security](SECURITY.md) and [changelog](CHANGELOG.md). Reproductions with a public accession and a small expected-output fixture are welcome.

## License

[MIT](LICENSE) © 2024–2026 Sebastian Boehler. Independent open-source software; not affiliated with or endorsed by the SEC.
