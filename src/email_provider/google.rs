use crate::{
    email_builder::Email,
    email_transmission::{SendEmail, SentEmail, Status},
    utils::format_green,
};
use anyhow::{Context, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use reqwest::{header, Client as ReqwestClient};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleResponse {
    id: String,
}

pub struct GoogleClient {
    client: ReqwestClient,
}

impl GoogleClient {
    pub fn new() -> Result<Self, anyhow::Error> {
        let token = env::var("GOOGLE_OAUTH_TOKEN")
            .context("Missing environment variable 'GOOGLE_OAUTH_TOKEN'")?;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()?;

        println!(
            "Connecting to Google server ... {}",
            format_green("ok")
        );

        Ok(GoogleClient { client })
    }
}

impl<'a> SendEmail<'a> for GoogleClient {
    #[tokio::main]
    async fn send(&self, email: &'a Email<'a>) -> Result<SentEmail<'a>, anyhow::Error> {
        let message = URL_SAFE_NO_PAD.encode(email.mime_format.message.formatted());
        let request_body = serde_json::json!({ "raw": message });

        let response = self
            .client
            .post("https://gmail.googleapis.com/gmail/v1/users/me/messages/send")
            .json(&request_body)
            .send()
            .await;

        let status = match response {
            Ok(res) => {
                if res.status().is_success() {
                    let google_res: GoogleResponse = res.json().await?;
                    Status::SentOk(google_res.id)
                } else {
                    let err_body = res.text().await?;
                    Status::SentError(err_body)
                }
            }
            Err(err) => Status::SentError(err.to_string()),
        };
        let sent_email = SentEmail::new(email, status);

        Ok(sent_email)
    }
}
