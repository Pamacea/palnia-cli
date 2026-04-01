use std::fs;

use anyhow::{Context, Result};
use colored::Colorize;

const PALNIA_MD_CONTENT: &str = r#"# Palnia CLI

Outil en ligne de commande pour interagir avec l'app Palnia (tâches, événements, habitudes, agenda).

## Installation

```bash
npm install -g @oalacea/palnia-cli
# ou
cargo install --git https://github.com/Pamacea/palnia-cli
```

## Authentification

```bash
palnia login                    # Connexion interactive (token plt_...)
palnia login --url https://...  # Avec URL API custom
palnia logout                   # Déconnexion
palnia whoami                   # Utilisateur courant
```

## Commandes

### Tâches

```bash
palnia tasks                    # Lister les tâches actives
palnia tasks all                # Toutes les tâches (y compris done)
palnia tasks add "Titre" -c professionnel -p urgent --due 2026-04-01
palnia tasks done <id>          # Marquer comme terminée
palnia tasks doing <id>         # Marquer en cours
palnia tasks delete <id>        # Supprimer
palnia tasks subtask <id> "Sous-tâche"
palnia tasks update <id>        # Modifier une tâche
palnia tasks archive <id>       # Archiver
palnia tasks archived           # Lister archivées
```

### Événements

```bash
palnia events                   # Événements du jour
palnia events week              # Événements de la semaine
palnia events add "Réunion" --date 2026-04-01 --start 14:00 --end 15:00 -c professionnel
palnia events delete <id>
palnia events update <id>       # Modifier un événement
```

### Habitudes

```bash
palnia habits                   # Lister les habitudes
palnia habits add "Méditation" -c santé
palnia habits toggle <id>       # Cocher/décocher aujourd'hui
palnia habits delete <id>
```

### Agenda

```bash
palnia agenda                   # Agenda du jour (événements + tâches)
palnia agenda week              # Agenda de la semaine
```

### Images

```bash
palnia images list              # Galerie d'images
palnia images show <id>         # Détails d'une image
palnia images upload <file> --task <id>    # Attacher à une tâche
palnia images download <id> --format webp  # Télécharger
palnia images quota             # Espace de stockage
palnia images rename <id> "Nouveau nom"
palnia images delete <id>
```

## Configuration

- **Fichier:** `~/.palnia/config.toml`
- **Variable d'env:** `PALNIA_API_URL` (URL de l'API)
- **Token:** Préfixé `plt_`, obtenu depuis les paramètres de Palnia
- **URL par défaut:** `https://palnia.newalfox.fr/api`
"#;

pub fn claude_code() -> Result<()> {
    let home = dirs::home_dir()
        .context("Impossible de déterminer le répertoire home")?;
    let global_claude_dir = home.join(".claude");
    let palnia_md_path = global_claude_dir.join("PALNIA.md");
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

    // 2. Write ~/.claude/PALNIA.md
    if palnia_md_path.exists() {
        println!(
            "{} {} existe déjà, écrasement…",
            "⟳".yellow().bold(),
            "~/.claude/PALNIA.md".bold()
        );
    }
    fs::write(&palnia_md_path, PALNIA_MD_CONTENT)
        .with_context(|| format!("Impossible d'écrire {}", palnia_md_path.display()))?;
    println!(
        "{} {} généré",
        "✓".green().bold(),
        "~/.claude/PALNIA.md".bold()
    );

    // 3. Add @PALNIA.md reference to ~/.claude/CLAUDE.md
    if claude_md_path.exists() {
        let content = fs::read_to_string(&claude_md_path)?;
        if content.contains("@PALNIA.md") {
            println!(
                "{} Référence {} déjà présente dans ~/.claude/CLAUDE.md",
                "–".dimmed(),
                "@PALNIA.md".bold()
            );
        } else {
            let updated = format!("{}\n\n@PALNIA.md\n", content.trim_end());
            fs::write(&claude_md_path, updated)?;
            println!(
                "{} Référence {} ajoutée à ~/.claude/CLAUDE.md",
                "✓".green().bold(),
                "@PALNIA.md".bold()
            );
        }
    } else {
        fs::write(&claude_md_path, "@PALNIA.md\n")?;
        println!(
            "{} ~/.claude/CLAUDE.md créé avec référence {}",
            "✓".green().bold(),
            "@PALNIA.md".bold()
        );
    }

    println!();
    println!(
        "{}",
        "Claude Code est maintenant configuré pour Palnia !"
            .green()
            .bold()
    );

    Ok(())
}
