# Local parsing benchmark

Measured 8 September 2026 on an Apple M4 Max, macOS 26.6.2 arm64. Seven fresh processes per implementation; medians reported. Rust 1.98.0 optimized release build versus published npm 2.0.0 on Node 24.20.0.

| Measurement | npm 2.0.0 | Rust 3.0.0 |
| --- | ---: | ---: |
| HTML parsing and table extraction | 65.73 ms | 8.43 ms |
| Peak process resident memory | 174.36 MiB | 13.72 MiB |
| Extracted tables | 40 | 40 |

Every row and cell matched, not just the counts. On this fixture the Rust parser was about 7.8× faster and peak process memory was about 92% lower. This is one document on one machine, not a corpus benchmark or a promise about service latency or hosting bills. Process memory includes runtime/import overhead; timed parsing excludes imports, disk reads, network access and JSON serialization. These are cold-process runs, not warmed JIT throughput measurements.

Fixture: Apple's 10-Q, accession `0000320193-26-000020`, 1,018,210 bytes. [Public SEC document](https://www.sec.gov/Archives/edgar/data/320193/000032019326000020/aapl-20260627.htm).

SHA-256: `4ad5bea67cedfa7542d623900355cc8d143ef95c1acc135a597f2eedabdb9177`.

## Reproduce

Download the document once using your identifying User-Agent, then use the same bytes for both implementations. Do not include download time:

```sh
cargo build --locked --release --example profile_tables
/usr/bin/time -l target/release/examples/profile_tables filing.html
```

`profile_tables` reads the file before starting its timer and retains the result until the process ends. Repeat in seven separate processes. On macOS `/usr/bin/time -l` reports peak resident bytes; Linux uses different flags and units.

For the npm baseline, install `sec-data-fetcher@2.0.0` in a separate temporary directory and run this script with Node 24:

```js
import fs from 'node:fs';
import { SECClient } from 'sec-data-fetcher';
const html = fs.readFileSync(process.argv[2], 'utf8');
const client = new SECClient({ userAgent: 'offline-benchmark local@example.com' });
const start = performance.now();
const tables = client.extractTablesFromContent(html);
console.log(JSON.stringify({ parseMs: performance.now() - start, tables: tables.length }));
```

This baseline makes no requests. For output parity, serialize the baseline's `tables` and compare parsed JSON with `sec-data-fetcher tables --file filing.html`.

The Rust live smoke separately completed four SEC requests in 1.55 seconds during local verification. That includes deliberately spaced requests at two per second. The network and SEC publication schedule remain separate from parsing performance. Standard GitHub-hosted runners are free for public repositories; Rust compilation is not a CI cost-saving claim.
