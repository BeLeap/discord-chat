use std::fmt;

use async_trait::async_trait;
use reqwest::{header, Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::SECRETS;

use super::chattable::Chattable;

pub struct Cohere {
    client: Client,
}

impl Cohere {
    pub fn new() -> Cohere {
        let secrets = SECRETS.lock().unwrap();
        let token = secrets.get("COHERE_API_TOKEN").unwrap();

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client");

        Cohere { client }
    }
}

#[derive(Debug)]
pub struct CohereError {
    kind: CohereErrorKind,
}

impl fmt::Display for CohereError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kind: {}", self.kind)
    }
}

#[derive(Debug)]
enum CohereErrorKind {
    FailedToRequest,
    Non200Response,
    MalformedResponse,
}

impl fmt::Display for CohereErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Serialize)]
struct CohereRequest {
    prompt: String,
    max_tokens: u32,
    return_likelihoods: ReturnLikelihoods,
}

#[derive(Debug, Serialize)]
enum ReturnLikelihoods {
    NONE,
}

#[derive(Debug, Deserialize)]
struct CohereResponse {
    generations: Vec<CohereGeneration>,
}

#[derive(Debug, Deserialize)]
struct CohereGeneration {
    text: String,
}

#[async_trait]
impl Chattable for Cohere {
    type Error = CohereError;

    async fn chat(&self, instruction: String) -> Result<String, CohereError> {
        let prompt = format!(
            "Transcript of a dialog, where the User interacts with an Assistant named Bob. Bob is helpful, kind, honest, good at writing, and never fails to answer the User's requests immediately and with precision.

User: Hello, Bob.
Bob: Hello. How may I help you today?
User: Please tell me the largest city in Europe.
Bob: Sure. The largest city in Europe is Moscow, the capital of Russia.
User: {}
Bob:",
            instruction
        );
        let request = CohereRequest {
            prompt,
            max_tokens: 300,
            return_likelihoods: ReturnLikelihoods::NONE,
        };

        let request = self
            .client
            .post("https://api.cohere.ai/v1/generate")
            .json(&request);
        println!("{:#?}", request);

        let response = request.send().await;
        if let Err(e) = response {
            eprintln!("Failed to request: {}", e);
            return Err(CohereError {
                kind: CohereErrorKind::FailedToRequest,
            });
        }
        let response = response.unwrap();

        if response.status() != StatusCode::OK {
            eprintln!("{:#?}", response);
            return Err(CohereError {
                kind: CohereErrorKind::Non200Response,
            });
        }

        let response = response.json::<CohereResponse>().await;
        println!("{:#?}", response);
        if let Err(e) = response {
            eprintln!("Failed to deserialize response: {}", e);
            return Err(CohereError {
                kind: CohereErrorKind::MalformedResponse,
            });
        }
        let response = response.unwrap();

        Ok(response.generations[0].text.clone())
    }
}
