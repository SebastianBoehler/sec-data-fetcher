# Architecture

The repository contains one Rust implementation, exposed as a library and native CLI.

- `client.rs` owns HTTP access and endpoint methods. Clones share reqwest's connection pool and an `Arc` rate limiter. Redirects and automatic retries are disabled so requests remain explicit and paced.
- `limiter.rs` serializes request starts using Tokio time and a mutex. It spaces requests rather than sending bursts; cancellation does not reserve future slots.
- `cik.rs`, `submissions.rs`, `facts.rs` and `filing.rs` define validated identifiers, typed responses and filing selection. Selection returns metadata; downloading a body is an explicit operation.
- `parsers/tables.rs` uses HTML5 parsing and assigns rows/cells to their nearest table/row, avoiding duplicate nested rows.
- `parsers/xml.rs` uses roxmltree 0.20 and an iterative conversion into an ordered namespace-aware tree. Depth is bounded and DTDs are disabled. Version 0.21.1 introduced a recursive parsing path that failed the excessive-depth regression; upgrades must retain that test.
- `cli.rs` declares commands; `main.rs` handles input, JSON output and errors. Its Tokio runtime uses one thread. Local parsing does not construct an HTTP client.

There is no service, database, hidden retry layer or JavaScript bridge. Network requests and complete document parsing remain memory-resident. Benchmarks must distinguish local parsing from upstream SEC latency and identify their fixture, compiler mode and hardware.

Tests cover HTTP status preservation, headers, shared pacing, URL validation, CIK normalization, parallel filing columns, dates, numeric precision, XML semantics, nested HTML and CLI failures. The live example separately checks actual SEC access.
