use anyhow::Result;
use chrono::Local;
use clap::Subcommand;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

use crate::client::Client;
use crate::types::{CreateEvent, CreateEventSubtask, Event, ImportEvent, ImportEvents, RecurrenceRule, UpdateEvent};

/// Expand ~ to home directory in path
fn expand_home(path: &str) -> String {
    if path.starts_with("~/") || path == "~" {
        if let Some(home) = dirs::home_dir() {
            let rest = &path[1..]; // Skip ~
            return home.join(rest).to_string_lossy().to_string();
        }
    }
    path.to_string()
}

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
        /// Recurrence type (daily, weekly, monthly, quarterly, semiannual, yearly)
        #[arg(long)]
        recurrence: Option<String>,
        /// Recurrence interval (e.g. 1 = every, 2 = every 2...)
        #[arg(long)]
        recurrence_interval: Option<u32>,
        /// Recurrence end date (YYYY-MM-DD)
        #[arg(long)]
        recurrence_end: Option<String>,
        /// Days of week for weekly recurrence (0=Sun, 1=Mon, ..., 6=Sat)
        #[arg(long, value_delimiter = ',')]
        recurrence_days: Option<Vec<u8>>,
        /// Reminder in minutes before event
        #[arg(long)]
        reminder: Option<i32>,
        /// Attach an image file (supports ~ for home directory)
        #[arg(long)]
        image: Option<String>,
        /// End date for multi-day events (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Excluded dates for recurring events (comma-separated YYYY-MM-DD)
        #[arg(long, value_delimiter = ',')]
        exclude_dates: Option<Vec<String>>,
        /// ID of the event this is detached from
        #[arg(long)]
        detached_from_id: Option<String>,
        /// Subtasks (comma-separated titles)
        #[arg(long, value_delimiter = ',')]
        subtasks: Option<Vec<String>>,
    },
    /// Update an existing event
    Update {
        /// Event ID (first chars)
        id: String,
        /// New title
        #[arg(long)]
        title: Option<String>,
        /// New date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
        /// New start time (HH:MM)
        #[arg(long)]
        start: Option<String>,
        /// New end time (HH:MM)
        #[arg(long)]
        end: Option<String>,
        /// New category
        #[arg(short, long)]
        category: Option<String>,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        /// New notes
        #[arg(short, long)]
        notes: Option<String>,
        /// New tags (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        /// Reminder in minutes before event
        #[arg(long)]
        reminder: Option<i32>,
        /// Toggle all-day
        #[arg(long)]
        all_day: Option<bool>,
        /// End date for multi-day events (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Excluded dates for recurring events (comma-separated YYYY-MM-DD)
        #[arg(long, value_delimiter = ',')]
        exclude_dates: Option<Vec<String>>,
        /// ID of the event this is detached from
        #[arg(long)]
        detached_from_id: Option<String>,
    },
    /// Import events from JSON file
    Import {
        /// JSON file path (supports ~ for home directory)
        file: String,
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
            recurrence,
            recurrence_interval,
            recurrence_end,
            recurrence_days,
            reminder,
            image,
            end_date,
            exclude_dates,
            detached_from_id,
            subtasks,
        }) => {
            add(
                &client,
                title,
                date,
                start,
                end,
                category,
                description,
                notes,
                tags,
                all_day,
                recurrence,
                recurrence_interval,
                recurrence_end,
                recurrence_days,
                reminder,
                image,
                end_date,
                exclude_dates,
                detached_from_id,
                subtasks,
            )
            .await
        }
        Some(EventAction::Update {
            id,
            title,
            date,
            start,
            end,
            category,
            description,
            notes,
            tags,
            reminder,
            all_day,
            end_date,
            exclude_dates,
            detached_from_id,
        }) => {
            update(
                &client,
                &id,
                title,
                date,
                start,
                end,
                category,
                description,
                notes,
                tags,
                reminder,
                all_day,
                end_date,
                exclude_dates,
                detached_from_id,
            )
            .await
        }
        Some(EventAction::Delete { id }) => delete(&client, &id).await,
        Some(EventAction::Import { file }) => import(&client, &file).await,
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
    let is_all_day = event.start_time == "00:00" && event.end_time == "23:59";
    let time = if is_all_day {
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
    let recurrence_str = if event.recurrence.is_some() { " ↻".cyan() } else { "".normal() };
    let reminder_str = if let Some(mins) = event.reminder_minutes {
        if mins >= 60 {
            format!(" ⏰{}h", mins / 60)
        } else {
            format!(" ⏰{}min", mins)
        }
    } else {
        String::new()
    };

    println!(
        "  {} {}{}{}{} {}{}",
        time.cyan(),
        event.title,
        cat_tag,
        recurrence_str,
        reminder_str.yellow(),
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
    recurrence: Option<String>,
    recurrence_interval: Option<u32>,
    recurrence_end: Option<String>,
    recurrence_days: Option<Vec<u8>>,
    reminder: Option<i32>,
    image: Option<String>,
    end_date: Option<String>,
    exclude_dates: Option<Vec<String>>,
    detached_from_id: Option<String>,
    subtasks: Option<Vec<String>>,
) -> Result<()> {
    if let Some(ref cat) = category {
        if !["spiritual", "personal", "professional"].contains(&cat.as_str()) {
            anyhow::bail!("Catégorie invalide: '{}'. Valeurs: spiritual, personal, professional", cat);
        }
    }

    // Build recurrence rule if provided
    let recurrence_rule = if let Some(ref rtype) = recurrence {
        let valid_types = [
            "daily",
            "weekly",
            "monthly",
            "quarterly",
            "semiannual",
            "yearly",
        ];
        if !valid_types.contains(&rtype.as_str()) {
            anyhow::bail!(
                "Type de récurrence invalide: '{}'. Valeurs: {}",
                rtype,
                valid_types.join(", ")
            );
        }

        // Validate days of week for weekly
        if rtype != "weekly" && recurrence_days.is_some() {
            anyhow::bail!("Les jours de la semaine ne sont valides que pour la récurrence 'weekly'");
        }
        if let Some(ref days) = recurrence_days {
            for &day in days {
                if day > 6 {
                    anyhow::bail!("Jour invalide: {}. Doit être 0-6 (0=Dimanche, 6=Samedi)", day);
                }
            }
        }

        Some(RecurrenceRule {
            recurrence_type: rtype.clone(),
            interval: recurrence_interval.unwrap_or(1),
            end_date: recurrence_end,
            days_of_week: recurrence_days,
        })
    } else {
        None
    };

    // Build subtasks if provided
    let event_subtasks: Option<Vec<CreateEventSubtask>> = subtasks.map(|titles| {
        titles
            .into_iter()
            .map(|title| CreateEventSubtask { title })
            .collect()
    });

    let event: Event = client
        .post(
            "/events",
            &CreateEvent {
                title,
                date,
                start_time: Some(start),
                end_time: Some(end),
                category,
                description,
                notes,
                tags,
                all_day: if all_day { Some(true) } else { None },
                recurrence: recurrence_rule,
                reminder_minutes: reminder,
                end_date,
                excluded_dates: exclude_dates,
                detached_from_id,
                subtasks: event_subtasks,
            },
        )
        .await?;

    let recurrence_note = if event.recurrence.is_some() {
        " (récurrent)"
    } else {
        ""
    };

    let multi_day_note = if event.end_date.is_some() {
        " (multi-jours)"
    } else {
        ""
    };

    let is_all_day = event.start_time == "00:00" && event.end_time == "23:59";

    println!(
        "{} Événement créé: {} le {} {}{}{}",
        "✓".green().bold(),
        event.title.bold(),
        event.date,
        if is_all_day {
            "(journée)".to_string()
        } else {
            format!("{}-{}", event.start_time, event.end_time)
        },
        recurrence_note,
        multi_day_note
    );

    // Upload image if provided
    if let Some(img_path) = image {
        let expanded = expand_home(&img_path);
        let path = PathBuf::from(&expanded);
        if path.exists() {
            #[derive(serde::Deserialize)]
            struct UploadResponse {
                original_name: String,
            }
            let resp: UploadResponse = client
                .upload(
                    &format!("/images?eventId={}", event.id),
                    &path,
                    &[],
                )
                .await?;
            println!("  {} Image attachée: {}", "📎".cyan(), resp.original_name);
        } else {
            println!("  {} Image non trouvée: {}", "⚠".yellow(), expanded);
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn update(
    client: &Client,
    id_prefix: &str,
    title: Option<String>,
    date: Option<String>,
    start: Option<String>,
    end: Option<String>,
    category: Option<String>,
    description: Option<String>,
    notes: Option<String>,
    tags: Option<Vec<String>>,
    reminder: Option<i32>,
    all_day: Option<bool>,
    end_date: Option<String>,
    exclude_dates: Option<Vec<String>>,
    detached_from_id: Option<String>,
) -> Result<()> {
    let events: Vec<Event> = client.get("/events").await?;
    let matched = find_by_prefix(&events, id_prefix)?;

    // Handle all_day toggle
    let (start_time, end_time) = if all_day == Some(true) {
        (Some("00:00".to_string()), Some("23:59".to_string()))
    } else if all_day == Some(false) {
        (start.map(Some).unwrap_or(Some("09:00".to_string())), end.map(Some).unwrap_or(Some("10:00".to_string())))
    } else {
        (start.map(Some).unwrap_or(None), end.map(Some).unwrap_or(None))
    };

    let updated: Event = client
        .patch(
            &format!("/events/{}", matched.id),
            &UpdateEvent {
                title,
                category,
                date,
                start_time,
                end_time,
                description,
                notes,
                tags,
                recurrence: None, // Not supported in update for now
                reminder_minutes: reminder,
                end_date,
                excluded_dates: exclude_dates,
                detached_from_id,
            },
        )
        .await?;

    println!(
        "{} Événement mis à jour: {}",
        "✓".green().bold(),
        updated.title.bold()
    );
    Ok(())
}

async fn delete(client: &Client, id_prefix: &str) -> Result<()> {
    let events: Vec<Event> = client.get("/events").await?;
    let matched = find_by_prefix(&events, id_prefix)?;
    client.delete(&format!("/events/{}", matched.id)).await?;
    println!(
        "{} Événement supprimé: {}",
        "✓".green().bold(),
        matched.title
    );
    Ok(())
}

async fn import(client: &Client, file_path: &str) -> Result<()> {
    let expanded = expand_home(file_path);
    let path = PathBuf::from(&expanded);

    if !path.exists() {
        anyhow::bail!("Fichier non trouvé: {}", expanded);
    }

    let content = fs::read_to_string(&path)?;

    // Try to parse as ImportEvents wrapper first
    if let Ok(import_data) = serde_json::from_str::<ImportEvents>(&content) {
        let count = import_data.events.len();
        let result: Vec<Event> = client.post("/events/import", &import_data).await?;
        println!(
            "{} {} événement(s) importé(s)",
            "✓".green().bold(),
            result.len()
        );
        for event in &result {
            println!("  - {}", event.title);
        }
        if result.len() != count {
            println!(
                "  {} Note: Seulement {} sur {} événements ont été créés",
                "⚠".yellow(),
                result.len(),
                count
            );
        }
        return Ok(());
    }

    // Try to parse as single ImportEvent
    if let Ok(single_event) = serde_json::from_str::<ImportEvent>(&content) {
        let result: Event = client.post("/events/import", &ImportEvents { events: vec![single_event] }).await?;
        println!(
            "{} Événement importé: {}",
            "✓".green().bold(),
            result.title.bold()
        );
        return Ok(());
    }

    // Try to parse as array of ImportEvent
    if let Ok(events_array) = serde_json::from_str::<Vec<ImportEvent>>(&content) {
        let count = events_array.len();
        let result: Vec<Event> = client.post("/events/import", &ImportEvents { events: events_array }).await?;
        println!(
            "{} {} événement(s) importé(s)",
            "✓".green().bold(),
            result.len()
        );
        for event in &result {
            println!("  - {}", event.title);
        }
        if result.len() != count {
            println!(
                "  {} Note: Seulement {} sur {} événements ont été créés",
                "⚠".yellow(),
                result.len(),
                count
            );
        }
        return Ok(());
    }

    anyhow::bail!(
        "Format JSON invalide. Attendu: {{\"events\": [...]}} ou [...] ou un seul événement"
    );
}

fn find_by_prefix<'a>(events: &'a [Event], prefix: &str) -> Result<&'a Event> {
    let matches: Vec<&Event> = events
        .iter()
        .filter(|e| e.id.starts_with(prefix))
        .collect();
    match matches.len() {
        0 => anyhow::bail!("Aucun événement trouvé avec le préfixe '{}'", prefix),
        1 => Ok(matches[0]),
        n => anyhow::bail!(
            "{} événements correspondent au préfixe '{}'. Précisez davantage.",
            n,
            prefix
        ),
    }
}
