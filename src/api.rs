use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::Config;

pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(cfg: &Config) -> Result<Self> {
        let base_url = cfg.server.url.trim_end_matches('/').to_string();
        let api_token = &cfg.auth.token;

        let mut headers = HeaderMap::new();
        let auth = HeaderValue::from_str(&format!("Token {api_token}"))
            .context("Invalid token format")?;
        headers.insert(AUTHORIZATION, auth);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { http, base_url })
    }

    /// Build a full URL from a relative API path like "/api/part/"
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let resp = self
            .http
            .get(self.url(path))
            .query(query)
            .send()
            .await
            .context("Request failed")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("API error {status}: {body}");
        }

        resp.json::<T>().await.context("Failed to parse response")
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let resp = self
            .http
            .post(self.url(path))
            .json(body)
            .send()
            .await
            .context("Request failed")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("API error {status}: {body}");
        }

        resp.json::<T>().await.context("Failed to parse response")
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let resp = self
            .http
            .patch(self.url(path))
            .json(body)
            .send()
            .await
            .context("Request failed")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("API error {status}: {body}");
        }

        resp.json::<T>().await.context("Failed to parse response")
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let resp = self
            .http
            .delete(self.url(path))
            .send()
            .await
            .context("Request failed")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("API error {status}: {body}");
        }

        Ok(())
    }
}
