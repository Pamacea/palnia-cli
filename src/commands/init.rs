use std::fs;

use anyhow::{Context, Result};
use colored::Colorize;

const PLANIA_MD_CONTENT: &str = r#"# Plania CLI

Outil en ligne de commande pour interagir avec l'app Plania (tâches, événements, habitudes, agenda).

## Installation

```bash
cargo install --path plania-cli
```

## Authentification

```bash
plania login                    # Connexion interactive (token plt_...)
plania login --url https://...  # Avec URL API custom
plania logout                   # Déconnexion
plania whoami                   # Utilisateur courant
```

## Commandes

### Tâches

```bash
plania tasks                    # Lister les tâches actives
plania tasks all                # Toutes les tâches (y compris done)
plania tasks add "Titre" -c professionnel -p urgent --due 2026-03-20
plania tasks done <id>          # Marquer comme terminée
plania tasks delete <id>        # Supprimer
plania tasks subtask <id> "Sous-tâche"
```

### Événements

```bash
plania events                   # Événements du jour
plania events add "Réunion" --date 2026-03-20 --start 14:00 --end 15:00 -c professionnel
plania events delete <id>
```

### Habitudes

```bash
plania habits                   # Lister les habitudes
plania habits toggle <id>       # Cocher/décocher aujourd'hui
```

### Agenda

```bash
plania agenda                   # Agenda du jour (événements + tâches)
plania agenda week              # Agenda de la semaine
```

## Configuration

- **Fichier:** `~/.plania/config.toml`
- **Variable d'env:** `PLANIA_API_URL` (URL de l'API)
- **Token:** Préfixé `plt_`, obtenu depuis les paramètres de Plania
"#;

pub fn claude_code() -> Result<()> {
    let home = dirs::home_dir()
        .context("Impossible de déterminer le répertoire home")?;
    let global_claude_dir = home.join(".claude");
    let plania_md_path = global_claude_dir.join("PLANIA.md");
    let claude_md_path = global_claude_dir.join("CLAUDE.md");

    // 1. Create ~/.claude/ if needed
    if !global_claude_dir.exists() {
        fs::create_dir_all(&global_claude_dir)
            .with_context(|| format!("Impossible de créer {}", global_claude_dir.display()))?;
        println!(
            "{} Dossier {} créé",
            "✓".green().bold(),
            "~/.claude/".bold()
        );
    }

    // 2. Write ~/.claude/PLANIA.md
    if plania_md_path.exists() {
        println!(
            "{} {} existe déjà, écrasement…",
            "⟳".yellow().bold(),
            "~/.claude/PLANIA.md".bold()
        );
    }
    fs::write(&plania_md_path, PLANIA_MD_CONTENT)
        .with_context(|| format!("Impossible d'écrire {}", plania_md_path.display()))?;
    println!(
        "{} {} généré",
        "✓".green().bold(),
        "~/.claude/PLANIA.md".bold()
    );

    // 3. Add @PLANIA.md reference to ~/.claude/CLAUDE.md
    if claude_md_path.exists() {
        let content = fs::read_to_string(&claude_md_path)?;
        if content.contains("@PLANIA.md") {
            println!(
                "{} Référence {} déjà présente dans ~/.claude/CLAUDE.md",
                "–".dimmed(),
                "@PLANIA.md".bold()
            );
        } else {
            let updated = format!("{}\n\n@PLANIA.md\n", content.trim_end());
            fs::write(&claude_md_path, updated)?;
            println!(
                "{} Référence {} ajoutée à ~/.claude/CLAUDE.md",
                "✓".green().bold(),
                "@PLANIA.md".bold()
            );
        }
    } else {
        fs::write(&claude_md_path, "@PLANIA.md\n")?;
        println!(
            "{} ~/.claude/CLAUDE.md créé avec référence {}",
            "✓".green().bold(),
            "@PLANIA.md".bold()
        );
    }

    println!();
    println!(
        "{}",
        "Claude Code est maintenant configuré pour Plania !"
            .green()
            .bold()
    );

    Ok(())
}
