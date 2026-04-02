use anyhow::{Context, Result};
use chrono::Local;
use clap::Subcommand;
use colored::Colorize;

use crate::client::Client;
use crate::types::{CreateHabit, Habit, ImportHabits, UpdateHabit};

#[derive(Subcommand)]
pub enum HabitAction {
    /// Toggle a habit for today (or a specific date)
    Toggle {
        /// Habit ID (first chars)
        id: String,
        /// Date to toggle (YYYY-MM-DD), defaults to today
        #[arg(long)]
        date: Option<String>,
    },
    /// Add a new habit
    Add {
        /// Habit title
        title: String,
        /// Category: spiritual, personal, professional
        #[arg(short, long)]
        category: Option<String>,
        /// Frequency: daily, weekly
        #[arg(short, long)]
        frequency: Option<String>,
    },
    /// Delete a habit
    Delete {
        /// Habit ID (first chars)
        id: String,
    },
    /// Update a habit
    Update {
        /// Habit ID (first chars)
        id: String,
        /// New title
        #[arg(long)]
        title: Option<String>,
        /// New category: spiritual, personal, professional
        #[arg(long)]
        category: Option<String>,
        /// New frequency: daily, weekly
        #[arg(long)]
        frequency: Option<String>,
    },
    /// Import habits from a JSON file
    Import {
        /// Path to JSON file containing habits array
        file: String,
    },
}

pub async fn run(action: Option<HabitAction>) -> Result<()> {
    let client = Client::new()?;

    match action {
        None => list(&client).await,
        Some(HabitAction::Toggle { id, date }) => toggle(&client, &id, date).await,
        Some(HabitAction::Add {
            title,
            category,
            frequency,
        }) => add(&client, title, category, frequency).await,
        Some(HabitAction::Delete { id }) => delete(&client, &id).await,
        Some(HabitAction::Update {
            id,
            title,
            category,
            frequency,
        }) => update(&client, &id, title, category, frequency).await,
        Some(HabitAction::Import { file }) => import(&client, &file).await,
    }
}

async fn list(client: &Client) -> Result<()> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let habits: Vec<Habit> = client.get("/habits").await?;

    if habits.is_empty() {
        println!("Aucune habitude.");
        return Ok(());
    }

    println!("{}", "Habitudes".bold());
    for habit in &habits {
        let done_today = habit.completed_dates.contains(&today);
        let icon = if done_today {
            "✓".green().bold()
        } else {
            "○".dimmed()
        };
        let freq_tag = match habit.frequency.as_str() {
            "weekly" => " (hebdo)".dimmed(),
            _ => "".normal(),
        };
        let cat_tag = match habit.category.as_str() {
            "spiritual" => " ✦".magenta(),
            "professional" => " ●".blue(),
            _ => "".normal(),
        };
        let id_short = &habit.id[..8.min(habit.id.len())];
        println!("  {} {}{}{} {}", icon, habit.title, cat_tag, freq_tag, id_short.dimmed());
    }

    let done_count = habits
        .iter()
        .filter(|h| h.completed_dates.contains(&today))
        .count();
    println!(
        "\n  {}/{} complétées aujourd'hui",
        done_count.to_string().green().bold(),
        habits.len()
    );

    Ok(())
}

async fn toggle(client: &Client, id_prefix: &str, date: Option<String>) -> Result<()> {
    let target_date = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
    let habits: Vec<Habit> = client.get("/habits").await?;
    let matched = find_by_prefix(&habits, id_prefix)?;

    let was_done = matched.completed_dates.contains(&target_date);
    let _: Habit = client
        .post(
            &format!("/habits/{}/toggle", matched.id),
            &serde_json::json!({ "date": target_date }),
        )
        .await?;
    if was_done {
        println!("{} {} dé-cochée", "○".dimmed(), matched.title.bold());
    } else {
        println!("{} {} complétée !", "✓".green().bold(), matched.title.bold());
    }

    Ok(())
}

async fn add(
    client: &Client,
    title: String,
    category: Option<String>,
    frequency: Option<String>,
) -> Result<()> {
    if let Some(ref cat) = category {
        if !["spiritual", "personal", "professional"].contains(&cat.as_str()) {
            anyhow::bail!("Catégorie invalide: '{}'. Valeurs: spiritual, personal, professional", cat);
        }
    }
    if let Some(ref freq) = frequency {
        if !["daily", "weekly"].contains(&freq.as_str()) {
            anyhow::bail!("Fréquence invalide: '{}'. Valeurs: daily, weekly", freq);
        }
    }

    let habit: Habit = client
        .post("/habits", &CreateHabit { title, category, frequency })
        .await?;
    println!("{} Habitude créée: {}", "✓".green().bold(), habit.title.bold());
    Ok(())
}

async fn delete(client: &Client, id_prefix: &str) -> Result<()> {
    let habits: Vec<Habit> = client.get("/habits").await?;
    let matched = find_by_prefix(&habits, id_prefix)?;
    client.delete(&format!("/habits/{}", matched.id)).await?;
    println!("{} Habitude supprimée: {}", "✓".green().bold(), matched.title);
    Ok(())
}

fn find_by_prefix<'a>(habits: &'a [Habit], prefix: &str) -> Result<&'a Habit> {
    let matches: Vec<&Habit> = habits
        .iter()
        .filter(|h| h.id.starts_with(prefix))
        .collect();
    match matches.len() {
        0 => anyhow::bail!("Aucune habitude trouvée avec le préfixe '{}'", prefix),
        1 => Ok(matches[0]),
        n => anyhow::bail!(
            "{} habitudes correspondent au préfixe '{}'. Précisez davantage.",
            n,
            prefix
        ),
    }
}

async fn update(
    client: &Client,
    id_prefix: &str,
    title: Option<String>,
    category: Option<String>,
    frequency: Option<String>,
) -> Result<()> {
    if title.is_none() && category.is_none() && frequency.is_none() {
        anyhow::bail!("Au moins un champ doit être spécifié (title, category ou frequency)");
    }

    if let Some(ref cat) = category {
        if !["spiritual", "personal", "professional"].contains(&cat.as_str()) {
            anyhow::bail!("Catégorie invalide: '{}'. Valeurs: spiritual, personal, professional", cat);
        }
    }
    if let Some(ref freq) = frequency {
        if !["daily", "weekly"].contains(&freq.as_str()) {
            anyhow::bail!("Fréquence invalide: '{}'. Valeurs: daily, weekly", freq);
        }
    }

    let habits: Vec<Habit> = client.get("/habits").await?;
    let matched = find_by_prefix(&habits, id_prefix)?;

    let updated: Habit = client
        .patch(
            &format!("/habits/{}", matched.id),
            &UpdateHabit {
                title,
                category,
                frequency,
            },
        )
        .await?;

    println!("{} Habitude mise à jour: {}", "✓".green().bold(), updated.title.bold());
    Ok(())
}

async fn import(client: &Client, file_path: &str) -> Result<()> {
    let path = std::path::Path::new(file_path);
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Impossible de lire le fichier: {}", file_path))?;

    let import_data: ImportHabits = serde_json::from_str(&content)
        .with_context(|| format!("JSON invalide dans le fichier: {}", file_path))?;

    if import_data.habits.is_empty() {
        println!("Aucune habitude à importer (fichier vide ou tableau vide).");
        return Ok(());
    }

    let imported: Vec<Habit> = client.post("/habits/import", &import_data).await?;

    println!(
        "{} {} habitude(s) importée(s):",
        "✓".green().bold(),
        imported.len()
    );
    for habit in &imported {
        println!("  - {}", habit.title);
    }

    Ok(())
}
