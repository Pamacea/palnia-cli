use anyhow::Result;
use chrono::Local;
use clap::Subcommand;
use colored::Colorize;

use crate::client::Client;
use crate::types::{CreateEvent, Event};

#[derive(Subcommand)]
pub enum EventAction {
    /// Show this week's events
    Week,
    /// Add a new event
    Add {
        /// Event title
        title: String,
        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: String,
        /// Start time (HH:MM)
        #[arg(long, default_value = "09:00")]
        start: String,
        /// End time (HH:MM)
        #[arg(long, default_value = "10:00")]
        end: String,
        /// Category: spiritual, personal, professional
        #[arg(short, long)]
        category: Option<String>,
        /// Description
        #[arg(short, long)]
        description: Option<String>,
        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
        /// Tags (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        /// All day event (ignores --start and --end)
        #[arg(long)]
        all_day: bool,
    },
    /// Delete an event
    Delete {
        /// Event ID (first chars)
        id: String,
    },
}

pub async fn run(action: Option<EventAction>) -> Result<()> {
    let client = Client::new()?;

    match action {
        None => list_today(&client).await,
        Some(EventAction::Week) => list_week(&client).await,
        Some(EventAction::Add {
            title,
            date,
            start,
            end,
            category,
            description,
            notes,
            tags,
            all_day,
        }) => add(&client, title, date, start, end, category, description, notes, tags, all_day).await,
        Some(EventAction::Delete { id }) => delete(&client, &id).await,
    }
}

async fn list_today(client: &Client) -> Result<()> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let events: Vec<Event> = client.get("/events").await?;
    let today_events: Vec<&Event> = events.iter().filter(|e| e.date == today).collect();

    if today_events.is_empty() {
        println!("Aucun événement aujourd'hui.");
        return Ok(());
    }

    println!("{}", format!("Événements du {}", today).bold());
    for event in &today_events {
        print_event(event);
    }
    Ok(())
}

async fn list_week(client: &Client) -> Result<()> {
    let today = Local::now().date_naive();
    let week_end = today + chrono::Duration::days(7);
    let events: Vec<Event> = client.get("/events").await?;

    let week_events: Vec<&Event> = events
        .iter()
        .filter(|e| {
            if let Ok(d) = e.date.parse::<chrono::NaiveDate>() {
                d >= today && d < week_end
            } else {
                false
            }
        })
        .collect();

    if week_events.is_empty() {
        println!("Aucun événement cette semaine.");
        return Ok(());
    }

    println!("{}", "Événements de la semaine".bold());
    let mut current_date = String::new();
    for event in &week_events {
        if event.date != current_date {
            current_date = event.date.clone();
            println!("\n  {}", current_date.underline());
        }
        print!("  ");
        print_event(event);
    }
    Ok(())
}

fn print_event(event: &Event) {
    let time = if event.all_day {
        "Journée  ".to_string()
    } else {
        format!("{}-{}", event.start_time, event.end_time)
    };
    let cat_tag = match event.category.as_str() {
        "spiritual" => " ✦".magenta(),
        "professional" => " ●".blue(),
        _ => "".normal(),
    };
    let tags_str = if !event.tags.is_empty() {
        format!(" [{}]", event.tags.join(", "))
    } else {
        String::new()
    };
    let id_short = &event.id[..8.min(event.id.len())];
    println!(
        "  {} {}{}{} {}",
        time.cyan(),
        event.title,
        cat_tag,
        tags_str.dimmed(),
        id_short.dimmed(),
    );
}

#[allow(clippy::too_many_arguments)]
async fn add(
    client: &Client,
    title: String,
    date: String,
    start: String,
    end: String,
    category: Option<String>,
    description: Option<String>,
    notes: Option<String>,
    tags: Option<Vec<String>>,
    all_day: bool,
) -> Result<()> {
    if let Some(ref cat) = category {
        if !["spiritual", "personal", "professional"].contains(&cat.as_str()) {
            anyhow::bail!("Catégorie invalide: '{}'. Valeurs: spiritual, personal, professional", cat);
        }
    }

    let event: Event = client
        .post(
            "/events",
            &CreateEvent {
                title,
                date,
                start_time: start,
                end_time: end,
                category,
                description,
                notes,
                tags,
                all_day: if all_day { Some(true) } else { None },
            },
        )
        .await?;
    println!(
        "{} Événement créé: {} le {} {}",
        "✓".green().bold(),
        event.title.bold(),
        event.date,
        if event.all_day {
            "(journée)".to_string()
        } else {
            format!("{}-{}", event.start_time, event.end_time)
        }
    );
    Ok(())
}

async fn delete(client: &Client, id_prefix: &str) -> Result<()> {
    let events: Vec<Event> = client.get("/events").await?;
    let matches: Vec<&Event> = events.iter().filter(|e| e.id.starts_with(id_prefix)).collect();
    match matches.len() {
        0 => anyhow::bail!("Aucun événement trouvé avec le préfixe '{}'", id_prefix),
        1 => {
            client.delete(&format!("/events/{}", matches[0].id)).await?;
            println!(
                "{} Événement supprimé: {}",
                "✓".green().bold(),
                matches[0].title
            );
        }
        n => anyhow::bail!(
            "{} événements correspondent au préfixe '{}'. Précisez davantage.",
            n,
            id_prefix
        ),
    }
    Ok(())
}
