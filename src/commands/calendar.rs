use anyhow::Result;
use chrono::Local;
use clap::Subcommand;
use colored::Colorize;

use crate::client::Client;
use crate::types::{Event, Task};

#[derive(Subcommand)]
pub enum AgendaAction {
    /// Show this week's agenda
    Week,
}

pub async fn run(action: Option<AgendaAction>) -> Result<()> {
    let client = Client::new()?;

    match action {
        None => agenda_today(&client).await,
        Some(AgendaAction::Week) => agenda_week(&client).await,
    }
}

async fn agenda_today(client: &Client) -> Result<()> {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let events: Vec<Event> = client.get("/events").await?;
    let tasks: Vec<Task> = client.get("/tasks").await?;

    let today_events: Vec<&Event> = events.iter().filter(|e| e.date == today).collect();
    let active_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|t| t.status != "done")
        .collect();

    println!("{}", format!("Agenda du {}", today).bold().underline());

    if !today_events.is_empty() {
        println!("\n  {}", "Événements".cyan().bold());
        for event in &today_events {
            let is_all_day = event.start_time == "00:00" && event.end_time == "23:59";
            let time = if is_all_day {
                "Journée  ".to_string()
            } else {
                format!("{}-{}", event.start_time, event.end_time)
            };
            println!("    {} {}", time.cyan(), event.title);
        }
    }

    if !active_tasks.is_empty() {
        println!("\n  {}", "Tâches".yellow().bold());
        let doing: Vec<&&Task> = active_tasks.iter().filter(|t| t.status == "doing").collect();
        let todo: Vec<&&Task> = active_tasks.iter().filter(|t| t.status == "todo").collect();

        for task in doing {
            println!("    {} {}", "◉".yellow(), task.title.bold());
        }
        for task in todo.iter().take(5) {
            println!("    {} {}", "○".dimmed(), task.title);
        }
        if todo.len() > 5 {
            println!("    {} +{} autres", "…".dimmed(), todo.len() - 5);
        }
    }

    if today_events.is_empty() && active_tasks.is_empty() {
        println!("\n  Rien de prévu aujourd'hui.");
    }

    Ok(())
}

async fn agenda_week(client: &Client) -> Result<()> {
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

    println!("{}", "Agenda de la semaine".bold().underline());

    if week_events.is_empty() {
        println!("\n  Aucun événement cette semaine.");
    } else {
        let mut current_date = String::new();
        for event in &week_events {
            if event.date != current_date {
                current_date = event.date.clone();
                println!("\n  {}", current_date.bold());
            }
            let is_all_day = event.start_time == "00:00" && event.end_time == "23:59";
            let time = if is_all_day {
                "Journée  ".to_string()
            } else {
                format!("{}-{}", event.start_time, event.end_time)
            };
            println!("    {} {}", time.cyan(), event.title);
        }
    }

    Ok(())
}
