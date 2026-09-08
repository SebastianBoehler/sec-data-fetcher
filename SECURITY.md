# Security

Security maintenance targets Rust version 3 and newer. The historical npm implementation is no longer developed on this branch.

Report exploitable vulnerabilities through [GitHub private vulnerability reporting](https://github.com/SebastianBoehler/sec-data-fetcher/security/advisories/new). Include affected versions, impact and a minimal reproduction. Ordinary defects belong in public issues. Do not include credentials or private data.

Downloads accept only HTTPS SEC hosts, reject redirects and time out after 30 seconds. XML parsing rejects DTDs and limits output depth to 128. Documents and parsed trees still occupy memory: applications accepting untrusted input should enforce their own input-size and execution limits. The HTML parser follows HTML5 recovery rules and does not sanitize content for browser rendering.

The shared client limiter does not coordinate across independent processes or machines. Follow SEC access restrictions; do not bypass them with rotating identities or proxies.
