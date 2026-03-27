use anyhow::{bail, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use std::path::Path;

use crate::config;

pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new() -> Result<Self> {
        let auth = config::get_auth().context(
            "Non authentifié. Lancez `palnia login` d'abord.",
        )?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", auth.token))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
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
            bail!("Token expiré ou invalide. Relancez `palnia login`.");
        }
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            bail!("Erreur API ({}): {}", status, text);
        }
        Ok(())
    }

    /// Upload un fichier (multipart/form-data)
    pub async fn upload<T: DeserializeOwned>(
        &self,
        path: &str,
        file_path: &Path,
        extra_fields: &[(&str, &str)],
    ) -> Result<T> {
        let file_bytes = std::fs::read(file_path)
            .with_context(|| format!("Impossible de lire le fichier: {:?}", file_path))?;

        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("filename");

        let mime_type = mime_guess::from_path(file_path)
            .first_or_octet_stream()
            .to_string();

        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(file_bytes)
                .file_name(file_name.to_string())
                .mime_str(&mime_type)?,
        );

        for (key, value) in extra_fields {
            form = form.text(key.to_string(), value.to_string());
        }

        // Créer un client sans le header CONTENT_TYPE par défaut pour multipart
        let auth = config::get_auth().ok_or_else(|| anyhow::anyhow!("Non authentifié"))?;
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", auth.token))?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let resp = client
            .post(format!("{}{}", self.base_url, path))
            .multipart(form)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    /// Télécharge un fichier et l'écrit sur le disque
    pub async fn download(&self, path: &str, output_path: &Path) -> Result<()> {
        let auth = config::get_auth().ok_or_else(|| anyhow::anyhow!("Non authentifié"))?;
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", auth.token))?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let resp = client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;

        if resp.status() == 401 {
            bail!("Token expiré ou invalide. Relancez `palnia login`.");
        }
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            bail!("Erreur API ({}): {}", status, text);
        }

        let bytes = resp.bytes().await?;

        // Créer le dossier parent si nécessaire
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(output_path, bytes)
            .with_context(|| format!("Impossible d'écrire le fichier: {:?}", output_path))?;

        Ok(())
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T> {
        if resp.status() == 401 {
            bail!("Token expiré ou invalide. Relancez `palnia login`.");
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
