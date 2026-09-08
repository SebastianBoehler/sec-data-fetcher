# Runtime and efficiency

SEC Data Fetcher is a Node.js library with a TypeScript API. Axios handles HTTP, axios-rate-limit schedules requests per client, fast-xml-parser converts XML, and Cheerio extracts HTML table text. The published JavaScript is CommonJS and is checked from both CommonJS and ESM consumers.

Fetching is constrained by network access and the [SEC's shared 10 requests/second limit](https://www.sec.gov/about/developer-resources). Use one client per coordinated request budget and fetch metadata before choosing documents. Requests time out after 30 seconds; errors are returned to callers without automatic retries.

Parsing loads the input into memory. `getReports` also retains every matching document. For bounded workloads, retrieve metadata and process individual filings; use the SEC's bulk archives for full-history ingestion. The library does not expose a streaming parser or a multi-machine rate limiter.

A lower-level parser could be useful for a demonstrated archive-processing bottleneck. Before changing languages, measure CPU time, peak memory and correctness on representative filings separately from download time. A faster parser that loses nested-table content or source context is not an improvement. Keeping the existing npm integration avoids introducing native build and cross-platform packaging requirements without a measured benefit.
