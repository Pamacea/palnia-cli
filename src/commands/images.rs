use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;

use crate::client::Client;
use crate::types::{GalleryImage, ImageQuota};

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

/// Format file size human-readable
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

#[derive(Subcommand)]
pub enum ImageAction {
    /// List all images (gallery)
    List,
    /// Show detailed info about an image
    Show {
        /// Image ID (first chars)
        id: String,
    },
    /// Upload an image file
    Upload {
        /// Path to the image file (supports ~ for home directory)
        file: String,
        /// Attach to a task
        #[arg(long)]
        task: Option<String>,
        /// Attach to an event
        #[arg(long)]
        event: Option<String>,
    },
    /// Download an image
    Download {
        /// Image ID (first chars)
        id: String,
        /// Output path (default: current directory with original name)
        #[arg(short, long)]
        output: Option<String>,
        /// Format: webp or original (default: original)
        #[arg(long)]
        format: Option<String>,
    },
    /// Delete an image
    Delete {
        /// Image ID (first chars)
        id: String,
    },
    /// Rename an image
    Rename {
        /// Image ID (first chars)
        id: String,
        /// New name
        new_name: String,
    },
    /// Show storage quota
    Quota,
}

pub async fn run(action: Option<ImageAction>) -> Result<()> {
    let client = Client::new()?;

    match action {
        None => list(&client).await,
        Some(ImageAction::List) => list(&client).await,
        Some(ImageAction::Show { id }) => show(&client, &id).await,
        Some(ImageAction::Upload { file, task, event }) => {
            let expanded = expand_home(&file);
            upload(&client, PathBuf::from(&expanded), task, event).await
        }
        Some(ImageAction::Download { id, output, format }) => {
            download(&client, &id, output, format).await
        }
        Some(ImageAction::Delete { id }) => delete(&client, &id).await,
        Some(ImageAction::Rename { id, new_name }) => rename(&client, &id, &new_name).await,
        Some(ImageAction::Quota) => quota(&client).await,
    }
}

async fn list(client: &Client) -> Result<()> {
    let images: Vec<GalleryImage> = client.get("/images/all").await?;

    if images.is_empty() {
        println!("Aucune image.");
        return Ok(());
    }

    println!("  Galerie ({} images):\n", images.len());

    for img in &images {
        let id_short = &img.id[..8.min(img.id.len())];
        let size_str = format_size(img.size);

        // Extract extension from mime type
        let ext = img.mime_type.split('/').last().unwrap_or("bin");
        let ext_tag = match ext {
            "webp" => " [WebP]".cyan(),
            "jpeg" | "jpg" => " [JPG]".yellow(),
            "png" => " [PNG]".green(),
            "gif" => " [GIF]".magenta(),
            _ => format!("[{}]", ext).dimmed(),
        };

        let entity = if let Some(ref tid) = img.task_id {
            format!("Tâche: {}", tid[..8.min(tid.len())].cyan())
        } else if let Some(ref eid) = img.event_id {
            format!("Événement: {}", eid[..8.min(eid.len())].blue())
        } else if let Some(ref name) = img.entity_name {
            format!("({})", name.dimmed())
        } else {
            "Orpheline".dimmed().to_string()
        };

        // Format date
        let date = img.created_at
            .split('T')
            .next()
            .unwrap_or(&img.created_at);

        println!(
            "  {} {} {} {} {} - {}",
            id_short.dimmed(),
            img.original_name.bold(),
            ext_tag,
            size_str.dimmed(),
            entity,
            date.dimmed()
        );
    }

    Ok(())
}

async fn show(client: &Client, id_prefix: &str) -> Result<()> {
    let images: Vec<GalleryImage> = client.get("/images/all").await?;
    let matched = find_by_prefix(&images, id_prefix)?;

    println!("  Détails de l'image:\n");
    println!("    ID: {}", matched.id.bold());
    println!("    Nom: {}", matched.original_name.bold());
    println!("    Format: {}", matched.mime_type);
    println!("    Taille: {}", format_size(matched.size).bold());

    // Parse date
    if let Some(date_part) = matched.created_at.split('T').next() {
        if let Some(time_part) = matched.created_at.split('T').nth(1) {
            let time = time_part.split('.').next().unwrap_or(time_part);
            println!("    Ajoutée: {} à {}", date_part, time);
        }
    }

    if let Some(ref tid) = matched.task_id {
        println!("    Tâche associée: {}", tid.bold());
    }
    if let Some(ref eid) = matched.event_id {
        println!("    Événement associé: {}", eid.bold());
    }
    if let Some(ref name) = matched.entity_name {
        println!("    Entité: {}", name);
    }

    Ok(())
}

async fn upload(client: &Client, file: PathBuf, task: Option<String>, event: Option<String>) -> Result<()> {
    if !file.exists() {
        anyhow::bail!("Fichier introuvable: {:?}", file);
    }

    // Préparer les champs supplémentaires
    let mut extra_fields = Vec::new();
    if let Some(ref tid) = task {
        extra_fields.push(("taskId", tid.as_str()));
    }
    if let Some(ref eid) = event {
        extra_fields.push(("eventId", eid.as_str()));
    }

    #[derive(serde::Deserialize)]
    struct UploadResponse {
        original_name: String,
        size: u64,
        id: String,
    }

    let resp: UploadResponse = client
        .upload("/images", &file, &extra_fields)
        .await?;

    let size_mb = resp.size as f64 / (1024.0 * 1024.0);

    println!(
        "{} Image uploadée: {} ({:.2} MB)",
        "✓".green().bold(),
        resp.original_name.bold(),
        size_mb
    );
    println!("    ID: {}", resp.id[..8.min(resp.id.len())].dimmed());

    if task.is_some() {
        println!("    Attachée à la tâche: {}", task.unwrap().bold());
    }
    if event.is_some() {
        println!("    Attachée à l'événement: {}", event.unwrap().bold());
    }

    Ok(())
}

async fn download(client: &Client, id_prefix: &str, output: Option<String>, format: Option<String>) -> Result<()> {
    let images: Vec<GalleryImage> = client.get("/images/all").await?;
    let matched = find_by_prefix(&images, id_prefix)?;

    // Determine format
    let use_webp = match format.as_deref() {
        Some("webp") | Some("WebP") => true,
        Some("original") | Some("orig") => false,
        None => false,
        Some(f) => anyhow::bail!("Format invalide: '{}'. Utilisez 'webp' ou 'original'", f),
    };

    // Build output filename
    let filename = if use_webp {
        matched
            .original_name
            .rsplit('.')
            .next()
            .map(|base| format!("{}.webp", base))
            .unwrap_or_else(|| matched.original_name.clone())
    } else {
        matched.original_name.clone()
    };

    let output_path = if let Some(out) = output {
        let expanded = expand_home(&out);
        PathBuf::from(&expanded)
    } else {
        PathBuf::from(&filename)
    };

    let path = if use_webp {
        format!("/images/download/{}?format=webp", matched.id)
    } else {
        format!("/images/download/{}", matched.id)
    };

    client.download(&path, &output_path).await?;

    let format_note = if use_webp { " (WebP)" } else { "" };
    println!(
        "{} Image téléchargée: {}{}",
        "✓".green().bold(),
        output_path.display().to_string().bold(),
        format_note.dimmed()
    );

    Ok(())
}

async fn delete(client: &Client, id_prefix: &str) -> Result<()> {
    let images: Vec<GalleryImage> = client.get("/images/all").await?;
    let matched = find_by_prefix(&images, id_prefix)?;

    client.delete(&format!("/images/{}", matched.id)).await?;

    println!(
        "{} Image supprimée: {}",
        "✓".green().bold(),
        matched.original_name.bold()
    );

    Ok(())
}

async fn rename(client: &Client, id_prefix: &str, new_name: &str) -> Result<()> {
    let images: Vec<GalleryImage> = client.get("/images/all").await?;
    let matched = find_by_prefix(&images, id_prefix)?;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct RenameBody {
        #[serde(rename = "originalName")]
        original_name: String,
    }

    client
        .patch::<serde_json::Value, _>(
            &format!("/images/{}", matched.id),
            &RenameBody {
                original_name: new_name.to_string(),
            },
        )
        .await?;

    println!(
        "{} Image renommée: {} → {}",
        "✓".green().bold(),
        matched.original_name.bold(),
        new_name.bold()
    );

    Ok(())
}

async fn quota(client: &Client) -> Result<()> {
    let quota: ImageQuota = client.get("/images/quota").await?;

    let used_mb = quota.used as f64 / (1024.0 * 1024.0);
    let limit_mb = quota.limit as f64 / (1024.0 * 1024.0);
    let percentage = (quota.used as f64 / quota.limit as f64) * 100.0;

    let bar_width = 30;
    let filled = (percentage as f64 / 100.0 * bar_width as f64) as usize;
    let bar: String = (0..bar_width)
        .map(|i| if i < filled { '█' } else { '░' })
        .collect();

    let colored_bar: String = if percentage >= 90.0 {
        bar.red().to_string()
    } else if percentage >= 70.0 {
        bar.yellow().to_string()
    } else {
        bar.green().to_string()
    };

    println!("  Quota de stockage:\n");
    println!(
        "  {} {} / {} ({:.1}%)",
        colored_bar,
        format!("{:.1} MB", used_mb).bold(),
        format!("{:.1} MB", limit_mb).dimmed(),
        percentage
    );
    println!(
        "  {} images",
        quota.count.to_string().bold()
    );

    Ok(())
}

fn find_by_prefix<'a>(images: &'a [GalleryImage], prefix: &str) -> Result<&'a GalleryImage> {
    let matches: Vec<&GalleryImage> = images
        .iter()
        .filter(|i| i.id.starts_with(prefix))
        .collect();

    match matches.len() {
        0 => anyhow::bail!("Aucune image trouvée avec le préfixe '{}'", prefix),
        1 => Ok(matches[0]),
        n => anyhow::bail!(
            "{} images correspondent au préfixe '{}'. Précisez davantage.",
            n,
            prefix
        ),
    }
}
