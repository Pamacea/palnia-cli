use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::fs;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Structure pour la réponse GitHub API
#[derive(serde::Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

/// Vérifie si une mise à jour est disponible
pub async fn check_update() -> Result<Option<String>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let resp = client
        .get("https://api.github.com/repos/Pamacea/palnia-cli/releases/latest")
        .header("User-Agent", "palnia-cli")
        .send()
        .await?;

    if !resp.status().is_success() {
        // Silencieux en cas d'erreur réseau
        return Ok(None);
    }

    let release: GitHubRelease = resp.json().await?;
    let latest_version = release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name);

    if latest_version != CURRENT_VERSION {
        Ok(Some(format!(
            "v{} (actuel: v{})",
            latest_version, CURRENT_VERSION
        )))
    } else {
        Ok(None)
    }
}

/// Affiche une notification si une mise à jour est disponible (non bloquant)
pub async fn notify_update_available() {
    match check_update().await {
        Ok(Some(version)) => {
            println!(
                "\n{} {} disponible ! {}",
                "➜".yellow().bold(),
                "Nouvelle version".bold(),
                version.dimmed()
            );
            println!("{} Lancez {} pour mettre à jour.\n",
                "→".dimmed(),
                "`palnia update`".bold()
            );
        }
        _ => {}
    }
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
