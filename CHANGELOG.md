# Changelog

## 3.0.0 — 2026-09-08

Full replacement with a Rust library and native CLI. Removed TypeScript sources, npm tooling and Node CI from the active tree.

- Added typed SEC submissions and company facts, validated CIKs and filing URLs, shared request pacing, explicit HTTP errors and connection reuse.
- Added CLI commands for lookup, submissions, facts, report selection, document downloads and offline HTML/XML parsing.
- Changed report selection to metadata only, with explicit date filtering; download documents individually.
- Preserved nested HTML table ownership and introduced ordered XML output with namespace URIs and text-preserving identifiers.
- Added deterministic Rust/CLI tests, package verification, live SEC smoke and dependency maintenance.

See [migration notes](docs/migration-v3.md). The previous npm release remains available separately; version 3 is not an npm package.


## 2.0.0 — 2026-09-08

A maintenance release for the SEC EDGAR filings and financial-data toolkit.

### Migration from 1.x

- Node.js 22 or newer is required. CI covers Node 22, 24 and 26.
- Company submissions, company facts and generic parsed XML now have explicit exported types. Unknown submissions/XML fields require narrowing instead of implicit `any` access.
- `maxRequestsPerSecond` must be an integer between 1 and 10. Empty User-Agents and invalid CIKs throw before making requests.
- `fetchFiling` preserves the original request error, including Axios HTTP status information, instead of wrapping it in a plain `Error`.
- XML document-defined entities are no longer expanded. The XML parser dependency moves from major 4 to major 5; verify application-specific XML fixtures when upgrading.
- Nested table rows and cells are no longer duplicated in their parent table. Parent cell text still contains descendant text.

### Fixed and maintained

- Fixed the GitHub source build's CommonJS/ESM mismatch while retaining both `require` and named `import` usage.
- Normalize CIKs before constructing submissions and company-facts URLs.
- Remove random query strings from ticker lookups and trim ticker input.
- Add a 30-second HTTP timeout and derive raw-document Host headers from the requested URL.
- Update dependencies, restore the complete MIT license and document the actual parser and history limitations.
- Add deterministic regression tests, packed-package consumer tests, a separate live SEC smoke workflow, supported-Node CI and Dependabot.
- Refresh README examples, banner, badges, contribution guidance, security reporting and issue templates.

## 1.0.2 — 2024-09-13

Previous published npm release, including filing downloads and table extraction. Later GitHub-only configuration changes introduced the module-format mismatch fixed in 2.0.0.
