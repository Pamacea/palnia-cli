use anyhow::Result;
use colored::Colorize;
use dialoguer::{Input, Password};

use crate::client::Client;
use crate::config::{self, AuthConfig};
use crate::types::User;

const DEFAULT_API_URL: &str = "http://localhost:3001/api";

pub async fn login(url_override: Option<String>) -> Result<()> {
    let api_url = if let Some(url) = url_override {
        url
    } else if let Ok(env_url) = std::env::var("PLANIA_API_URL") {
        env_url
    } else {
        let input: String = Input::new()
            .with_prompt("URL de l'API")
            .default(DEFAULT_API_URL.to_string())
            .interact_text()?;
        input
    };

    let token: String = Password::new()
        .with_prompt("Token API (plt_...)")
        .interact()?;

    if !token.starts_with("plt_") {
        anyhow::bail!("Le token doit commencer par 'plt_'");
    }

    // Save config
    let mut cfg = config::load();
    cfg.auth = Some(AuthConfig {
        token,
        api_url,
    });
    config::save(&cfg)?;

    // Verify token by calling whoami
    match Client::new() {
        Ok(client) => match client.get::<User>("/users/me").await {
            Ok(user) => {
                println!(
                    "{} Connecté en tant que {} ({})",
                    "✓".green().bold(),
                    user.name.bold(),
                    user.email
                );
            }
            Err(_) => {
                config::clear_auth();
                anyhow::bail!("Token invalide ou expiré.");
            }
        },
        Err(e) => {
            config::clear_auth();
            anyhow::bail!("Erreur de connexion: {}", e);
        }
    }

    Ok(())
}

pub fn logout() -> Result<()> {
    config::clear_auth();
    println!("{} Déconnecté.", "✓".green().bold());
    Ok(())
}

pub async fn whoami() -> Result<()> {
    let client = Client::new()?;
    let user: User = client.get("/users/me").await?;
    println!("{} {}", "Nom:".bold(), user.name);
    println!("{} {}", "Email:".bold(), user.email);
    println!("{} {}", "Rôle:".bold(), user.role);
    Ok(())
}
