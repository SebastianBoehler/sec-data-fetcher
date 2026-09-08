# Contributing

Small, reproducible improvements to SEC data access and parsing are welcome.

## Development

Install Rust stable (minimum supported version: 1.88) and your platform's C/C++ build tools. Clone this repository and run:

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo package --locked
```

Tests run offline against fixtures and a loopback HTTP server. To check upstream access separately:

```sh
SEC_USER_AGENT='YourApp you@your-domain.com' cargo run --locked --release --example live_smoke
```

Use your real contact information. A SEC access error is not proof of a parser defect; include the status, public URL and environment in reports.

## Issues and pull requests

- Include the toolkit version, Rust version, operating system and a minimal reproduction.
- For parser defects, include a public accession/URL, a small fixture and expected output.
- Discuss significant public API changes first. Keep changes focused, use modules with clear ownership, and aim for files below 300 lines.
- Add focused regression tests for behavior changes; update documentation and migration notes when necessary.
- Use descriptive commits such as `fix(parser): preserve nested table ownership`.
- Do not commit credentials, contact details from your environment, downloaded archives or build output.

Contributions are licensed under MIT. Keep discussions respectful and focused on the code.

## Maintenance and releases

Dependabot checks Cargo dependencies weekly and Actions monthly. The XML dependency intentionally stays on 0.20: 0.21.1 overflowed a standard test-thread stack on the nested-input regression. Only upgrade after that regression passes without enlarging the stack.

1. Update `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md` with migration notes.
2. Run formatting, Clippy, tests, `cargo package --locked`, `cargo audit` and the live smoke. Check the minimum Rust toolchain too.
3. Push and wait for CI on the exact release commit.
4. Tag `vX.Y.Z` and create a GitHub release describing supported installation paths and breaking changes.
5. Verify installation from that tag in a clean location.

The Rust crate is not currently published on crates.io. A future registry release needs ownership/authentication, inspection of `cargo package --list`, successful package verification, and a clean registry install after publication. Do not publish Node packages from this Rust tree.

The weekly live workflow requires the `SEC_USER_AGENT` repository secret. Public hosted CI is free on standard GitHub runners; cache Rust builds to reduce execution time.
