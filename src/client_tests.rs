use super::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::oneshot,
};

async fn server(
    status: &str,
    body: &str,
    extra_headers: &str,
) -> (String, oneshot::Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/test", listener.local_addr().unwrap());
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n{extra_headers}\r\n{body}",
        body.len()
    );
    let (sender, receiver) = oneshot::channel();
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        loop {
            let mut buffer = [0; 1024];
            let size = socket.read(&mut buffer).await.unwrap();
            if size == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..size]);
            if request.windows(4).any(|part| part == b"\r\n\r\n") {
                break;
            }
        }
        let _ = sender.send(String::from_utf8(request).unwrap());
        socket.write_all(response.as_bytes()).await.unwrap();
    });
    (url, receiver)
}

#[test]
fn validates_client_options() {
    for agent in ["", " ", "Name\r\nInjected: value"] {
        assert!(SecClient::new(agent).is_err());
    }
    for rate in [0, 11, u32::MAX] {
        assert!(SecClient::with_rate_limit("Test test@example.com", rate).is_err());
    }
}

#[tokio::test]
async fn preserves_http_errors_and_does_not_follow_redirects() {
    for (status, code) in [
        ("403 Forbidden", 403),
        ("429 Too Many Requests", 429),
        ("302 Found", 302),
        ("500 Server Error", 500),
    ] {
        let (url, _) = server(
            status,
            "denied",
            "Location: http://127.0.0.1:1/should-not-follow\r\n",
        )
        .await;
        let client = SecClient::new("Test test@example.com").unwrap();
        assert!(
            matches!(client.request(&url).await, Err(Error::Http { status, .. }) if status == code)
        );
    }
}

#[tokio::test]
async fn sends_contact_headers_and_decodes_json() {
    let (url, headers) = server(
        "200 OK",
        r#"{"cik":320193}"#,
        "Content-Type: application/json\r\n",
    )
    .await;
    let client = SecClient::new("Test test@example.com").unwrap();
    let data: serde_json::Value = client.json(&url).await.unwrap();
    assert_eq!(data["cik"], 320193);
    let headers = headers.await.unwrap().to_ascii_lowercase();
    assert!(headers.contains("user-agent: test test@example.com"));
    let encoding = headers
        .lines()
        .find_map(|line| line.strip_prefix("accept-encoding:"))
        .unwrap();
    let mut encodings: Vec<_> = encoding.split(',').map(str::trim).collect();
    encodings.sort();
    assert_eq!(encodings, ["deflate", "gzip"]);
}

#[tokio::test]
async fn returns_structural_errors_for_invalid_json() {
    let (url, _) = server("200 OK", "not JSON", "").await;
    let client = SecClient::new("Test test@example.com").unwrap();
    assert!(matches!(
        client.json::<serde_json::Value>(&url).await,
        Err(Error::Json(_))
    ));
}

#[tokio::test]
async fn cloned_clients_share_the_request_budget() {
    let (first, _) = server("200 OK", "ok", "").await;
    let (second, _) = server("200 OK", "ok", "").await;
    let client = SecClient::with_rate_limit("Test test@example.com", 10).unwrap();
    let clone = client.clone();
    let start = tokio::time::Instant::now();
    client.request(&first).await.unwrap();
    clone.request(&second).await.unwrap();
    assert!(start.elapsed() >= Duration::from_millis(100));
}

#[tokio::test]
async fn rejects_unsafe_document_urls_before_any_request() {
    let client = SecClient::new("Test test@example.com").unwrap();
    for url in [
        "http://www.sec.gov/a",
        "https://www.sec.gov.evil.test/a",
        "https://user@www.sec.gov/a",
        "https://www.sec.gov:444/a",
        "file:///tmp/a",
        "https://127.0.0.1/a",
        "bad URL",
    ] {
        assert!(matches!(
            client.fetch_filing(url).await,
            Err(Error::InvalidInput(_))
        ));
    }
    assert!(validate_document_url("https://www.sec.gov/Archives/a.htm").is_ok());
    assert!(validate_document_url("https://data.sec.gov/submissions/CIK0000320193.json").is_ok());
    assert!(client.cik_lookup(" ").await.is_err());
}
