use std::process::Command;

fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sec-data-fetcher"))
        .args(args)
        .env_remove("SEC_USER_AGENT")
        .output()
        .unwrap()
}

#[test]
fn parses_local_files_without_network_credentials() {
    let output = cli(&["tables", "--file", "tests/fixtures/tables.html"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let tables: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(tables.as_array().unwrap().len(), 3);
    let output = cli(&["xml", "--file", "tests/fixtures/document.xml"]);
    assert!(output.status.success());
}

#[test]
fn rejects_missing_auth_invalid_inputs_and_ambiguous_sources() {
    for args in [
        vec!["lookup", "AAPL"],
        vec!["submissions", "../bad"],
        vec!["reports", "320193"],
        vec!["reports", "320193", "--after", "2026-02-30"],
        vec!["tables"],
        vec!["tables", "--file", "a", "--url", "https://www.sec.gov/a"],
        vec!["tables", "--file", "missing.html"],
    ] {
        let output = cli(&args);
        assert!(!output.status.success(), "accepted {args:?}");
        assert!(!output.stderr.is_empty());
        assert!(output.stdout.is_empty());
    }
}
