use anyhow::{bail, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;

use crate::config;

pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new() -> Result<Self> {
        let auth = config::get_auth().context(
            "Non authentifié. Lancez `plania login` d'abord.",
        )?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", auth.token))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self {
            http,
            base_url: auth.api_url,
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self
            .http
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;
        self.handle_response(resp).await
    }

    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let resp = self
            .http
            .post(format!("{}{}", self.base_url, path))
            .json(body)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    pub async fn patch<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let resp = self
            .http
            .patch(format!("{}{}", self.base_url, path))
            .json(body)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let resp = self
            .http
            .delete(format!("{}{}", self.base_url, path))
            .send()
            .await?;
        if resp.status() == 401 {
            bail!("Token expiré ou invalide. Relancez `plania login`.");
        }
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            bail!("Erreur API ({}): {}", status, text);
        }
        Ok(())
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T> {
        if resp.status() == 401 {
            bail!("Token expiré ou invalide. Relancez `plania login`.");
        }
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            bail!("Erreur API ({}): {}", status, text);
        }
        let data = resp.json::<T>().await?;
        Ok(data)
    }
}
