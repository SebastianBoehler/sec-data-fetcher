# Contributing

Help make SEC filings and financial data easier to work with. Small, reproducible fixes are welcome.

## Local development

Use Node.js 22+ and npm. Clone the repository, then run:

```sh
npm ci
npm run check
```

The check runs lint, deterministic tests, a TypeScript build and installs an npm tarball into a temporary consumer project. It verifies CommonJS, ESM and TypeScript entry points. Unit tests use controlled fixtures and never contact the SEC.

To test real SEC access separately:

```sh
SEC_USER_AGENT='YourApp you@your-domain.com' npm run test:live
```

Use your own application name and real contact email. A 403 from SEC is an access failure, not proof of a parser defect. Include the HTTP status, environment and failing endpoint in reports, but remove credentials and private information.

## Issues and pull requests

- Search existing issues before opening one. Include package/Node versions, expected and actual behavior, and a minimal reproduction.
- For parsing issues, provide a public SEC URL/accession and the exact table or XML element. A small fixture and expected result are more useful than a large dump.
- Discuss new endpoints, parser behavior or public API changes in an issue before building a large change.
- Keep changes focused. Aim for files under 300 lines; put public interfaces in `src/types.ts`. Match existing style.
- Add a focused regression test for behavior changes. Update the README and changelog if users need to change code.
- Use descriptive commits such as `fix(parser): avoid duplicated nested rows`.
- Do not commit contact details from your environment, credentials, downloaded filing archives, `dist/` or `node_modules/`.

By contributing, you agree that your contributions are licensed under this project's MIT license. Be respectful, address the code rather than the person, and keep discussions constructive.

## Maintenance and releases

Dependency updates arrive through Dependabot. Review changes and run `npm run check`; green tests alone do not establish XML parsing correctness on arbitrary filings. The live smoke workflow runs weekly and can be dispatched manually. Maintainers must configure the `SEC_USER_AGENT` repository secret.

Release procedure:

1. Document changes and migration requirements in `CHANGELOG.md`; set the package version and regenerate the lockfile.
2. Run `npm ci`, `npm run check`, `npm audit --omit=dev` and the live smoke. Investigate failures before release.
3. Push the reviewed commits and wait for CI on the exact commit.
4. Confirm npm ownership/authentication with `npm whoami` and `npm owner ls sec-data-fetcher`.
5. Run `npm publish --dry-run`, inspect the package contents, then `npm publish`.
6. Verify the registry version and install it into a clean consumer for import and live smoke checks.
7. Tag that commit as `vX.Y.Z` and create a GitHub Release with the changelog and migration notes.

Use a major version for breaking API/type changes or an increased minimum Node version. Never overwrite a published version; ship a new version to correct a bad release.
