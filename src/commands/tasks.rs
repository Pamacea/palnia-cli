use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;

use crate::client::Client;
use crate::types::{AddSubtask, CreateTask, RecurrenceRule, Subtask, Task, UpdateTask};

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
pub enum TaskAction {
    /// Add a new task
    Add {
        /// Task title
        title: String,
        /// Category: spiritual, personal, professional
        #[arg(short, long)]
        category: Option<String>,
        /// Priority: urgent, normal, low
        #[arg(short, long)]
        priority: Option<String>,
        /// Due date (YYYY-MM-DD)
        #[arg(short, long)]
        due: Option<String>,
        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
        /// Tags (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        /// Subtasks (comma-separated titles)
        #[arg(short, long, value_delimiter = ',')]
        subtasks: Option<Vec<String>>,
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
        /// Attach an image file (supports ~ for home directory)
        #[arg(long)]
        image: Option<String>,
    },
    /// Mark a task as done
    Done {
        /// Task ID (first chars)
        id: String,
    },
    /// Mark a task as doing
    Doing {
        /// Task ID (first chars)
        id: String,
    },
    /// Update a task
    Update {
        /// Task ID (first chars)
        id: String,
        /// New title
        #[arg(long)]
        title: Option<String>,
        /// New category
        #[arg(short, long)]
        category: Option<String>,
        /// New priority
        #[arg(short, long)]
        priority: Option<String>,
        /// New due date (YYYY-MM-DD)
        #[arg(short, long)]
        due: Option<String>,
        /// New notes
        #[arg(short, long)]
        notes: Option<String>,
        /// New tags (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        /// Mark as todo/doing/done
        #[arg(long)]
        status: Option<String>,
        /// Archive/unarchive
        #[arg(long)]
        archived: Option<bool>,
    },
    /// Delete a task
    Delete {
        /// Task ID (first chars)
        id: String,
    },
    /// Show all tasks (including done)
    All,
    /// Show archived tasks
    Archived,
    /// Archive a task
    Archive {
        /// Task ID (first chars)
        id: String,
    },
    /// Unarchive a task
    Unarchive {
        /// Task ID (first chars)
        id: String,
    },
    /// Add a subtask to an existing task
    Subtask {
        /// Task ID (first chars)
        id: String,
        /// Subtask title
        title: String,
    },
}

pub async fn run(action: Option<TaskAction>) -> Result<()> {
    let client = Client::new()?;

    match action {
        None => list(&client, false, false).await,
        Some(TaskAction::Add {
            title,
            category,
            priority,
            due,
            notes,
            tags,
            subtasks,
            recurrence,
            recurrence_interval,
            recurrence_end,
            recurrence_days,
            image,
        }) => add(
            &client,
            title,
            category,
            priority,
            due,
            notes,
            tags,
            subtasks,
            recurrence,
            recurrence_interval,
            recurrence_end,
            recurrence_days,
            image,
        )
        .await,
        Some(TaskAction::Done { id }) => set_status(&client, &id, "done").await,
        Some(TaskAction::Doing { id }) => set_status(&client, &id, "doing").await,
        Some(TaskAction::Update {
            id,
            title,
            category,
            priority,
            due,
            notes,
            tags,
            status,
            archived,
        }) => update(&client, &id, title, category, priority, due, notes, tags, status, archived).await,
        Some(TaskAction::Delete { id }) => delete(&client, &id).await,
        Some(TaskAction::All) => list(&client, true, false).await,
        Some(TaskAction::Archived) => list(&client, true, true).await,
        Some(TaskAction::Archive { id }) => set_archive(&client, &id, true).await,
        Some(TaskAction::Unarchive { id }) => set_archive(&client, &id, false).await,
        Some(TaskAction::Subtask { id, title }) => add_subtask(&client, &id, title).await,
    }
}

async fn list(client: &Client, show_all: bool, only_archived: bool) -> Result<()> {
    let tasks: Vec<Task> = client.get("/tasks").await?;

    let filtered: Vec<&Task> = if only_archived {
        tasks.iter().filter(|t| t.archived).collect()
    } else if show_all {
        tasks.iter().filter(|t| !t.archived).collect()
    } else {
        tasks
            .iter()
            .filter(|t| t.status != "done" && !t.archived)
            .collect()
    };

    if filtered.is_empty() {
        println!(
            "Aucune tâche{}.",
            if only_archived {
                " archivée"
            } else if show_all {
                ""
            } else {
                " en cours"
            }
        );
        return Ok(());
    }

    for task in &filtered {
        let status_icon = match task.status.as_str() {
            "done" => "✓".green(),
            "doing" => "◉".yellow(),
            "todo" => "○".white(),
            _ => "?".white(),
        };
        let priority_tag = match task.priority.as_str() {
            "urgent" => " [!]".red().bold(),
            "low" => " [↓]".dimmed(),
            _ => "".normal(),
        };
        let cat_tag = match task.category.as_str() {
            "spiritual" => " ✦".magenta(),
            "professional" => " ●".blue(),
            _ => "".normal(),
        };
        let archive_tag = if task.archived { " [archived]".dimmed() } else { "".normal() };
        let id_short = &task.id[..8.min(task.id.len())];
        let due_str = if let Some(ref d) = task.due_date {
            format!(" ({})", d)
        } else {
            String::new()
        };
        let tags_str = if !task.tags.is_empty() {
            format!(" [{}]", task.tags.join(", "))
        } else {
            String::new()
        };
        let recurrence_str = if task.recurrence.is_some() {
            " ↻".cyan()
        } else {
            "".normal()
        };

        println!(
            "  {} {}{}{}{}{} {} {}{} ",
            status_icon,
            task.title.bold(),
            priority_tag,
            cat_tag,
            recurrence_str,
            archive_tag,
            due_str.dimmed(),
            tags_str.dimmed(),
            format!(" {}", id_short).dimmed(),
        );

        // Show subtasks if present
        if let Some(ref subtasks) = task.subtasks {
            for st in subtasks {
                let st_icon = if st.done { "✓".green() } else { "·".dimmed() };
                println!("      {} {}", st_icon, st.title);
            }
        }
    }

    if !show_all && !only_archived {
        let doing = filtered.iter().filter(|t| t.status == "doing").count();
        let todo = filtered.iter().filter(|t| t.status == "todo").count();
        println!(
            "\n  {} en cours, {} à faire",
            doing.to_string().yellow().bold(),
            todo.to_string().white().bold()
        );
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn add(
    client: &Client,
    title: String,
    category: Option<String>,
    priority: Option<String>,
    due: Option<String>,
    notes: Option<String>,
    tags: Option<Vec<String>>,
    subtasks: Option<Vec<String>>,
    recurrence: Option<String>,
    recurrence_interval: Option<u32>,
    recurrence_end: Option<String>,
    recurrence_days: Option<Vec<u8>>,
    image: Option<String>,
) -> Result<()> {
    // Validate category if provided
    if let Some(ref cat) = category {
        if !["spiritual", "personal", "professional"].contains(&cat.as_str()) {
            anyhow::bail!("Catégorie invalide: '{}'. Valeurs: spiritual, personal, professional", cat);
        }
    }
    // Validate priority if provided
    if let Some(ref pri) = priority {
        if !["urgent", "normal", "low"].contains(&pri.as_str()) {
            anyhow::bail!("Priorité invalide: '{}'. Valeurs: urgent, normal, low", pri);
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

    let task: Task = client
        .post(
            "/tasks",
            &CreateTask {
                title,
                category,
                priority,
                due_date: due,
                notes,
                tags,
                recurrence: recurrence_rule,
            },
        )
        .await?;

    println!("{} Tâche créée: {}", "✓".green().bold(), task.title.bold());

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
                    &format!("/images?taskId={}", task.id),
                    &path,
                    &[],
                )
                .await?;
            println!("  {} Image attachée: {}", "📎".cyan(), resp.original_name);
        } else {
            println!("  {} Image non trouvée: {}", "⚠".yellow(), expanded);
        }
    }

    // Add subtasks if provided
    if let Some(subtask_titles) = subtasks {
        for st_title in subtask_titles {
            let st_title = st_title.trim().to_string();
            if st_title.is_empty() {
                continue;
            }
            let _: Subtask = client
                .post(
                    &format!("/tasks/{}/subtasks", task.id),
                    &AddSubtask { title: st_title.clone() },
                )
                .await?;
            println!("  {} Sous-tâche: {}", "+".cyan(), st_title);
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn update(
    client: &Client,
    id_prefix: &str,
    title: Option<String>,
    category: Option<String>,
    priority: Option<String>,
    due: Option<String>,
    notes: Option<String>,
    tags: Option<Vec<String>>,
    status: Option<String>,
    archived: Option<bool>,
) -> Result<()> {
    let tasks: Vec<Task> = client.get("/tasks").await?;
    let matched = find_by_prefix(&tasks, id_prefix)?;

    // Validate status if provided
    if let Some(ref s) = status {
        if !["todo", "doing", "done"].contains(&s.as_str()) {
            anyhow::bail!("Statut invalide: '{}'. Valeurs: todo, doing, done", s);
        }
    }

    let updated: Task = client
        .patch(
            &format!("/tasks/{}", matched.id),
            &UpdateTask {
                title,
                category,
                priority,
                due_date: due,
                notes,
                tags,
                status,
                archived,
                recurrence: None, // Not supported in update for now
            },
        )
        .await?;

    println!(
        "{} Tâche mise à jour: {}",
        "✓".green().bold(),
        updated.title.bold()
    );
    Ok(())
}

async fn set_status(client: &Client, id_prefix: &str, status: &str) -> Result<()> {
    let tasks: Vec<Task> = client.get("/tasks").await?;
    let matched = find_by_prefix(&tasks, id_prefix)?;
    let _: Task = client
        .patch(
            &format!("/tasks/{}", matched.id),
            &UpdateTask {
                status: Some(status.to_string()),
                title: None,
                category: None,
                priority: None,
                notes: None,
                tags: None,
                due_date: None,
                archived: None,
                recurrence: None,
            },
        )
        .await?;
    let label = match status {
        "done" => "terminée",
        "doing" => "en cours",
        _ => status,
    };
    println!(
        "{} {} marquée comme {}",
        "✓".green().bold(),
        matched.title.bold(),
        label
    );
    Ok(())
}

async fn set_archive(client: &Client, id_prefix: &str, archived: bool) -> Result<()> {
    let tasks: Vec<Task> = client.get("/tasks").await?;
    let matched = find_by_prefix(&tasks, id_prefix)?;
    let _: Task = client
        .patch(
            &format!("/tasks/{}", matched.id),
            &UpdateTask {
                status: None,
                title: None,
                category: None,
                priority: None,
                notes: None,
                tags: None,
                due_date: None,
                archived: Some(archived),
                recurrence: None,
            },
        )
        .await?;
    let action = if archived { "archivée" } else { "désarchivée" };
    println!(
        "{} {} {}",
        "✓".green().bold(),
        matched.title.bold(),
        action
    );
    Ok(())
}

async fn delete(client: &Client, id_prefix: &str) -> Result<()> {
    let tasks: Vec<Task> = client.get("/tasks").await?;
    let matched = find_by_prefix(&tasks, id_prefix)?;
    client.delete(&format!("/tasks/{}", matched.id)).await?;
    println!("{} Tâche supprimée: {}", "✓".green().bold(), matched.title);
    Ok(())
}

async fn add_subtask(client: &Client, id_prefix: &str, title: String) -> Result<()> {
    let tasks: Vec<Task> = client.get("/tasks").await?;
    let matched = find_by_prefix(&tasks, id_prefix)?;
    let _: Subtask = client
        .post(
            &format!("/tasks/{}/subtasks", matched.id),
            &AddSubtask { title: title.clone() },
        )
        .await?;
    println!(
        "{} Sous-tâche ajoutée à {}: {}",
        "✓".green().bold(),
        matched.title.bold(),
        title
    );
    Ok(())
}

fn find_by_prefix<'a>(tasks: &'a [Task], prefix: &str) -> Result<&'a Task> {
    let matches: Vec<&Task> = tasks
        .iter()
        .filter(|t| t.id.starts_with(prefix))
        .collect();
    match matches.len() {
        0 => anyhow::bail!("Aucune tâche trouvée avec le préfixe '{}'", prefix),
        1 => Ok(matches[0]),
        n => anyhow::bail!(
            "{} tâches correspondent au préfixe '{}'. Précisez davantage.",
            n,
            prefix
        ),
    }
}
