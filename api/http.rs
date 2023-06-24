use std::time::Duration;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

fn get_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.58"));
    reqwest::Client::builder().default_headers(headers).timeout(Duration::from_secs(5)).build().unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(
            Body::Text(get_client().get("https://www.google.com.sg/").send().await?.text().await?)
        )?)
}