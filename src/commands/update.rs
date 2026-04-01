use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::fs;
use std::process::Command;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Structure pour la réponse GitHub API
#[derive(serde::Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

/// Auto-update silencieux : télécharge et installe si une nouvelle version est disponible
/// Renvoie true si une mise à jour a été effectuée
pub async fn auto_update_silent() -> bool {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    let resp = match client
        .get("https://api.github.com/repos/Pamacea/palnia-cli/releases/latest")
        .header("User-Agent", "palnia-cli")
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return false,
    };

    if !resp.status().is_success() {
        return false;
    }

    let release: GitHubRelease = match resp.json().await {
        Ok(r) => r,
        Err(_) => return false,
    };

    let latest_version = release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name);

    if latest_version == CURRENT_VERSION {
        return false;
    }

    // Installation npm : afficher uniquement un message
    if is_npm_installation() {
        println!(
            "\n{} {} → {}\n",
            "⟳".yellow().bold(),
            CURRENT_VERSION.dimmed(),
            latest_version.green().bold()
        );
        println!("{} Lancez: {}",
            "→".dimmed(),
            "`npm update -g @oalacea/palnia-cli`".bold().cyan()
        );
        println!();
        return false; // Pas d'auto-update pour npm
    }

    // Auto-update pour cargo/standalone
    let (platform, arch) = if cfg!(windows) {
        ("windows", "x86_64-pc-windows-msvc")
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            ("darwin", "aarch64-apple-darwin")
        } else {
            ("darwin", "x86_64-apple-darwin")
        }
    } else {
        if cfg!(target_arch = "aarch64") {
            ("linux", "aarch64-unknown-linux-gnu")
        } else {
            ("linux", "x86_64-unknown-linux-gnu")
        }
    };

    let extension = if platform == "windows" { ".exe" } else { "" };
    let asset_name = format!("palnia-{}-{}{}", latest_version, arch, extension);
    let download_url = format!(
        "https://github.com/Pamacea/palnia-cli/releases/download/v{}/{}",
        latest_version, asset_name
    );

    let resp = match client.get(&download_url).send().await {
        Ok(r) => r,
        Err(_) => return false,
    };

    if !resp.status().is_success() {
        return false;
    }

    let bytes = match resp.bytes().await {
        Ok(b) => b,
        Err(_) => return false,
    };

    let current_exe = match env::current_exe() {
        Ok(e) => e,
        Err(_) => return false,
    };

    let current_exe = fs::canonicalize(&current_exe).unwrap_or(current_exe);

    // Sauvegarde
    let backup_path = current_exe.with_extension("old");
    let _ = fs::copy(&current_exe, &backup_path);

    // Écrire le nouveau binaire
    if fs::write(&current_exe, &bytes).is_err() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(mut perms) = fs::metadata(&current_exe).map(|m| m.permissions()) {
            perms.set_mode(0o755);
            let _ = fs::set_permissions(&current_exe, perms);
        }
    }

    println!(
        "\n{} {} → {}\n",
        "✓ Mis à jour !".green().bold(),
        CURRENT_VERSION.dimmed(),
        latest_version.green().bold()
    );

    // Nettoyer la sauvegarde
    let backup_path_clone = backup_path.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let _ = fs::remove_file(backup_path_clone);
    });

    true
}

/// Relance le binaire actuel avec les mêmes arguments
pub fn restart_with_new_version() {
    let current_exe = match env::current_exe() {
        Ok(e) => e,
        Err(_) => return,
    };

    let args: Vec<String> = env::args().skip(1).collect();

    let _ = Command::new(&current_exe)
        .args(&args)
        .spawn();
}

/// Vérifie si le CLI est installé via npm
fn is_npm_installation() -> bool {
    if let Ok(exe) = env::current_exe() {
        if let Some(path) = exe.to_str() {
            return path.contains(".palnia") && path.contains("bin");
        }
    }
    false
}

/// Met à jour le CLI vers la dernière version
pub async fn update() -> Result<()> {
    println!("{} Vérification des mises à jour...", "⟳".yellow().bold());

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let resp = client
        .get("https://api.github.com/repos/Pamacea/palnia-cli/releases/latest")
        .header("User-Agent", "palnia-cli")
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Impossible de contacter GitHub pour vérifier les mises à jour");
    }

    let release: GitHubRelease = resp.json().await?;
    let latest_version = release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name);

    if latest_version == CURRENT_VERSION {
        println!("{} Vous êtes déjà à jour ({})", "✓".green().bold(), CURRENT_VERSION);
        return Ok(());
    }

    println!(
        "{} {} → {}",
        "Mise à jour disponible:".yellow().bold(),
        CURRENT_VERSION.dimmed(),
        latest_version.green().bold()
    );

    // Pour installation npm, on ne peut pas remplacer le binaire en cours d'exécution sur Windows
    if is_npm_installation() {
        println!("\n{} Installation détectée: npm", "ℹ".dimmed());
        println!("\nPour mettre à jour, lancez:");
        println!("  {} {}", "`npm update -g @oalacea/palnia-cli`".bold().cyan(),
                 "ou".dimmed());
        println!("  {} {}\n", "`npm install -g @oalacea/palnia-cli@latest`".bold().cyan(),
                 "pour forcer la réinstallation.".dimmed());
        return Ok(());
    }

    // Pour cargo ou développement, on essaie de remplacer le binaire
    let (platform, arch) = if cfg!(windows) {
        ("windows", "x86_64-pc-windows-msvc")
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            ("darwin", "aarch64-apple-darwin")
        } else {
            ("darwin", "x86_64-apple-darwin")
        }
    } else {
        if cfg!(target_arch = "aarch64") {
            ("linux", "aarch64-unknown-linux-gnu")
        } else {
            ("linux", "x86_64-unknown-linux-gnu")
        }
    };

    let extension = if platform == "windows" { ".exe" } else { "" };
    let asset_name = format!("palnia-{}-{}{}", latest_version, arch, extension);
    let download_url = format!(
        "https://github.com/Pamacea/palnia-cli/releases/download/v{}/{}",
        latest_version, asset_name
    );

    println!("{} Téléchargement depuis GitHub...", "⟳".yellow().bold());

    let resp = client.get(&download_url).send().await?;

    if !resp.status().is_success() {
        println!(
            "\n{} Impossible de télécharger le binaire pour votre plateforme ({} {}).",
            "✗".red(),
            platform,
            arch
        );
        println!("Visitez: {}\n", release.html_url);
        anyhow::bail!("Binaire non disponible pour cette plateforme");
    }

    let bytes = resp.bytes().await?;

    // Trouver le chemin de l'exécutable actuel
    let current_exe = env::current_exe().context("Impossible de trouver l'exécutable")?;

    // Sur certains systèmes, le binaire peut être dans un lien symbolique
    let current_exe = fs::canonicalize(&current_exe)
        .unwrap_or(current_exe);

    // Créer une sauvegarde
    let backup_path = current_exe.with_extension("old");
    fs::copy(&current_exe, &backup_path)?;

    // Écrire le nouveau binaire
    fs::write(&current_exe, &bytes)
        .context("Impossible d'écrire le nouveau binaire")?;

    // Rendre exécutable sur Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&current_exe)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&current_exe, perms)?;
    }

    // Nettoyer la sauvegarde après un court délai
    let backup_path_clone = backup_path.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let _ = fs::remove_file(backup_path_clone);
    });

    println!(
        "\n{} {} → {}\n",
        "✓ Mis à jour avec succès !".green().bold(),
        CURRENT_VERSION.dimmed(),
        latest_version.green().bold()
    );

    Ok(())
}
