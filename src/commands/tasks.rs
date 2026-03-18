use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

use crate::client::Client;
use crate::types::{AddSubtask, CreateTask, Subtask, Task, UpdateTask};

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
    /// Delete a task
    Delete {
        /// Task ID (first chars)
        id: String,
    },
    /// Show all tasks (including done)
    All,
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
        None => list(&client, false).await,
        Some(TaskAction::Add {
            title,
            category,
            priority,
            due,
            notes,
            tags,
            subtasks,
        }) => add(&client, title, category, priority, due, notes, tags, subtasks).await,
        Some(TaskAction::Done { id }) => set_status(&client, &id, "done").await,
        Some(TaskAction::Doing { id }) => set_status(&client, &id, "doing").await,
        Some(TaskAction::Delete { id }) => delete(&client, &id).await,
        Some(TaskAction::All) => list(&client, true).await,
        Some(TaskAction::Subtask { id, title }) => add_subtask(&client, &id, title).await,
    }
}

async fn list(client: &Client, show_all: bool) -> Result<()> {
    let tasks: Vec<Task> = client.get("/tasks").await?;
    let filtered: Vec<&Task> = if show_all {
        tasks.iter().collect()
    } else {
        tasks.iter().filter(|t| t.status != "done").collect()
    };

    if filtered.is_empty() {
        println!("Aucune tâche{}.", if show_all { "" } else { " en cours" });
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
        println!(
            "  {} {}{}{} {} {}{} ",
            status_icon,
            task.title.bold(),
            priority_tag,
            cat_tag,
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

    if !show_all {
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
            },
        )
        .await?;

    println!("{} Tâche créée: {}", "✓".green().bold(), task.title.bold());

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

async fn set_status(client: &Client, id_prefix: &str, status: &str) -> Result<()> {
    let tasks: Vec<Task> = client.get("/tasks").await?;
    let matched = find_by_prefix(&tasks, id_prefix)?;
    let _: Task = client
        .patch(
            &format!("/tasks/{}", matched.id),
            &UpdateTask {
                status: Some(status.to_string()),
                priority: None,
                notes: None,
                tags: None,
                due_date: None,
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
