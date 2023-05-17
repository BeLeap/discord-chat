use core::fmt;

use async_trait::async_trait;
use reqwest::{header, Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::SECRETS;

use super::chattable::Chattable;

#[derive(Debug)]
pub struct Bard {
    client: Client,
}

impl Bard {
    pub fn new() -> Bard {
        let secrets = SECRETS.lock().unwrap();
        let token = secrets.get("BARD_API_TOKEN").unwrap();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
        headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client");

        Bard { client }
    }
}

#[derive(Debug, Serialize)]
struct BardRequest {
    input: String,
}

#[derive(Debug, Deserialize)]
struct BardResponse {
    output: String,
}

#[derive(Debug)]
enum BardErrorKind {
    MalformedResponse,
    Non200Response,
    FailedToRequest,
    FailedToSerializeBody,
}

impl fmt::Display for BardErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct BardError {
    kind: BardErrorKind,
}

impl fmt::Display for BardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kind: {}", self.kind)
    }
}

#[async_trait]
impl Chattable for Bard {
    type Error = BardError;

    async fn chat(&self, instruction: String) -> Result<String, BardError> {
        let request = BardRequest { input: instruction };
        let body = serde_json::to_string(&request);

        if let Ok(body) = body {
            let response = self
                .client
                .post("https://api.bardapi.dev/chat")
                .body(body)
                .send()
                .await;

            if let Ok(response) = response {
                if response.status() == StatusCode::OK {
                    if let Ok(bard_response) = response.json::<BardResponse>().await {
                        println!("{:#?}", bard_response);
                        Ok(bard_response.output)
                    } else {
                        Err(BardError {
                            kind: BardErrorKind::MalformedResponse,
                        })
                    }
                } else {
                    println!("{:#?}", response);
                    Err(BardError {
                        kind: BardErrorKind::Non200Response,
                    })
                }
            } else {
                Err(BardError {
                    kind: BardErrorKind::FailedToRequest,
                })
            }
        } else {
            Err(BardError {
                kind: BardErrorKind::FailedToSerializeBody,
            })
        }
    }
}
