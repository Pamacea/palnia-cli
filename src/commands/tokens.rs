use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

use crate::client::Client;
use crate::types::{ApiToken, CreateToken};

#[derive(Subcommand)]
pub enum TokenAction {
    /// Add a new API token
    Add {
        /// Token name (e.g., "My Laptop", "CI/CD")
        name: String,
        /// Expiration in days (optional, defaults to server config)
        #[arg(long)]
        expires_in_days: Option<u32>,
    },
    /// Revoke a token
    Delete {
        /// Token ID (first chars)
        id: String,
    },
}

pub async fn run(action: Option<TokenAction>) -> Result<()> {
    let client = Client::new()?;

    match action {
        None => list(&client).await,
        Some(TokenAction::Add {
            name,
            expires_in_days,
        }) => add(&client, name, expires_in_days).await,
        Some(TokenAction::Delete { id }) => delete(&client, &id).await,
    }
}

async fn list(client: &Client) -> Result<()> {
    let tokens: Vec<ApiToken> = client.get("/tokens").await?;

    if tokens.is_empty() {
        println!("Aucun token API.");
        println!("\n  Créez un token avec:");
        println!("  {} palnia tokens add \"Nom du token\"", ">".dimmed());
        return Ok(());
    }

    println!("{}", "Tokens API".bold());
    println!();

    for token in &tokens {
        let id_short = &token.id[..8.min(token.id.len())];
        let expires_display = match &token.expires_at {
            Some(date) => {
                if let Ok(exp) = chrono::DateTime::parse_from_rfc3339(date) {
                    let now = chrono::Utc::now();
                    let days_left = (exp.with_timezone(&chrono::Utc) - now).num_days();
                    if days_left < 0 {
                        " (expiré)".red().bold().to_string()
                    } else if days_left < 7 {
                        format!(" (expire dans {}j)", days_left).yellow().to_string()
                    } else {
                        format!(" (expire dans {}j)", days_left).dimmed().to_string()
                    }
                } else {
                    "".to_string()
                }
            }
            None => " (pas d'expiration)".dimmed().to_string(),
        };

        let last_used = match &token.last_used_at {
            Some(date) => {
                if let Ok(used) = chrono::DateTime::parse_from_rfc3339(date) {
                    let now = chrono::Utc::now();
                    let days_ago = (now - used.with_timezone(&chrono::Utc)).num_days();
                    match days_ago {
                        0 => " (aujourd'hui)".dimmed().to_string(),
                        1 => " (hier)".dimmed().to_string(),
                        n if n < 30 => format!(" (il y a {}j)", n).dimmed().to_string(),
                        _ => "".to_string(),
                    }
                } else {
                    "".to_string()
                }
            }
            None => " (jamais utilisé)".dimmed().to_string(),
        };

        println!("  {} {}{}", "●".blue(), token.name.bold(), expires_display);
        println!("    {} {}{}", "Prefix:".dimmed(), token.prefix.dimmed(), last_used);
        println!("    {} {}", "ID:".dimmed(), id_short.dimmed());
        println!();
    }

    println!("  {} tokens actifs", tokens.len().to_string().cyan());

    Ok(())
}

async fn add(client: &Client, name: String, expires_in_days: Option<u32>) -> Result<()> {
    let create = CreateToken {
        name,
        expires_in_days,
    };

    let token: ApiToken = client.post("/tokens", &create).await?;

    println!("{} Token API créé avec succès!", "✓".green().bold());
    println!();
    println!("  Nom: {}", token.name.bold());
    println!("  Prefix: {}", token.prefix.dimmed());
    if let Some(exp) = &token.expires_at {
        println!("  Expiration: {}", exp.dimmed());
    }
    println!();
    if let Some(full_token) = &token.token {
        println!("  {}", "Token (conservez-le précieusement, il ne sera plus affiché):".yellow().bold());
        println!("  {}", full_token.cyan().bold());
        println!();
        println!("  {}", "Utilisation:".dimmed());
        println!("  {} palnia login --url https://palnia.newalfox.fr/api", "$".dimmed());
        println!("  {} palnia login {}", "$".dimmed(), full_token.cyan());
    }

    Ok(())
}

async fn delete(client: &Client, id_prefix: &str) -> Result<()> {
    let tokens: Vec<ApiToken> = client.get("/tokens").await?;
    let matched = find_by_prefix(&tokens, id_prefix)?;

    client.delete(&format!("/tokens/{}", matched.id)).await?;
    println!(
        "{} Token révoqué: {}",
        "✓".green().bold(),
        matched.name.bold()
    );

    Ok(())
}

fn find_by_prefix<'a>(tokens: &'a [ApiToken], prefix: &str) -> Result<&'a ApiToken> {
    let matches: Vec<&ApiToken> = tokens
        .iter()
        .filter(|t| t.id.starts_with(prefix))
        .collect();

    match matches.len() {
        0 => anyhow::bail!("Aucun token trouvé avec le préfixe '{}'", prefix),
        1 => Ok(matches[0]),
        n => anyhow::bail!(
            "{} tokens correspondent au préfixe '{}'. Précisez davantage.",
            n,
            prefix
        ),
    }
}
