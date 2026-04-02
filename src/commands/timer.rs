use anyhow::{Context, Result};
use clap::Subcommand;
use colored::Colorize;
use serde_json::Value as JsonValue;

use crate::client::Client;

#[derive(Subcommand)]
pub enum TimerAction {
    /// Get timer state for a type
    Get {
        /// Timer type (e.g., pomodoro, break)
        timer_type: String,
    },
    /// Set timer state for a type
    Set {
        /// Timer type (e.g., pomodoro, break)
        timer_type: String,
        /// JSON state (e.g., '{"timeLeft":1500,"isActive":true}')
        json: String,
    },
}

pub async fn run(action: Option<TimerAction>) -> Result<()> {
    let client = Client::new()?;

    match action {
        None => {
            anyhow::bail!("Spécifiez une action: get ou set. Ex: palnia timer get pomodoro");
        }
        Some(TimerAction::Get { timer_type }) => get(&client, &timer_type).await,
        Some(TimerAction::Set { timer_type, json }) => set(&client, &timer_type, &json).await,
    }
}

async fn get(client: &Client, timer_type: &str) -> Result<()> {
    let state: JsonValue = client
        .get(&format!("/timer-state/{}", timer_type))
        .await?;

    // Pretty print JSON
    let formatted = serde_json::to_string_pretty(&state)?;
    println!("{}", formatted);

    Ok(())
}

async fn set(client: &Client, timer_type: &str, json: &str) -> Result<()> {
    // Parse and validate JSON
    let value: JsonValue = serde_json::from_str(json)
        .with_context(|| format!("JSON invalide: '{}'", json))?;

    // Send PUT request
    let state: JsonValue = client
        .put(&format!("/timer-state/{}", timer_type), &value)
        .await?;

    println!("{} État sauvegardé pour '{}'", "✓".green().bold(), timer_type.bold());

    // Pretty print saved state
    let formatted = serde_json::to_string_pretty(&state)?;
    println!("{}", formatted);

    Ok(())
}
