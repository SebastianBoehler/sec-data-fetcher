# Security

Security maintenance targets the latest major release. Older versions may contain vulnerable dependencies; upgrade to the current release before reporting an issue.

Report exploitable vulnerabilities privately through [GitHub private vulnerability reporting](https://github.com/SebastianBoehler/sec-data-fetcher/security/advisories/new). Include affected versions, impact and a minimal reproduction. Do not include credentials, personal data or an exploit against someone else's service. Ordinary bugs belong in public issues.

Applications are responsible for validating user-supplied URLs before passing them to download methods. Those methods make server-side HTTP requests and are intended for trusted SEC document URLs. The toolkit loads documents and tables into memory; avoid accepting arbitrary large inputs from untrusted users. XML document-defined entity expansion is disabled.

SEC access restrictions and rate limits are documented in the README. They must not be bypassed with rotating identities or proxies.
