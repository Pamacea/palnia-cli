# Palnia CLI

CLI Rust pour [Palnia](https://palnia.newalfox.fr), l'app de productivité tout-en-un.

## Installation

```bash
# Via npm (recommandé)
npm install -g @oalacea/palnia-cli

# Ou via cargo
cargo install palnia-cli
```

## Authentification

Le CLI utilise des **tokens API** générés depuis l'app Palnia (Paramètres > Tokens API).

```bash
palnia login
# Coller votre token API (plt_...)
# URL par défaut : https://palnia.newalfox.fr/api

palnia whoami    # Vérifier la connexion
palnia logout    # Se déconnecter
```

Configuration stockée dans `~/.palnia/config.toml`. L'URL de l'API est configurable via `--url` ou la variable d'env `PALNIA_API_URL`.

## Commandes

### Tâches

```bash
palnia tasks                          # Lister les tâches actives (todo/doing)
palnia tasks all                      # Lister toutes les tâches (inclut done)
palnia tasks add "Titre"              # Créer une tâche
palnia tasks add "Titre" \
  -c professional \                   # Catégorie: spiritual, personal, professional
  -p urgent \                         # Priorité: urgent, normal, low
  --due 2026-03-20 \                  # Date d'échéance
  -n "Notes ici" \                    # Notes
  -t cli,rust \                       # Tags (séparés par virgule)
  -s "Sous-tâche 1,Sous-tâche 2"     # Sous-tâches

palnia tasks doing <id>               # Marquer en cours
palnia tasks done <id>                # Marquer terminée
palnia tasks delete <id>              # Supprimer
palnia tasks subtask <id> "Titre"     # Ajouter une sous-tâche
```

### Événements

```bash
palnia events                         # Événements du jour
palnia events week                    # Événements de la semaine
palnia events add "Titre" \
  --date 2026-03-20 \                 # Date (requis)
  --start 14:00 \                     # Heure de début (défaut: 09:00)
  --end 15:30 \                       # Heure de fin (défaut: 10:00)
  -c professional \                   # Catégorie
  -d "Description" \                  # Description
  -n "Notes" \                        # Notes
  -t meeting,demo \                   # Tags
  --all-day                           # Événement journée entière

palnia events delete <id>             # Supprimer
```

### Habitudes

```bash
palnia habits                         # Lister + statut du jour
palnia habits add "Titre"             # Créer une habitude
palnia habits add "Titre" \
  -c spiritual \                      # Catégorie
  -f weekly                           # Fréquence: daily, weekly

palnia habits toggle <id>             # Cocher/décocher aujourd'hui
palnia habits toggle <id> --date 2026-03-17  # Cocher une date spécifique
palnia habits delete <id>             # Supprimer
```

### Agenda

```bash
palnia agenda                         # Vue combinée du jour (événements + tâches)
palnia agenda week                    # Vue de la semaine
```

### Images

```bash
palnia images                         # Lister la galerie
palnia images upload <file>           # Upload une image
palnia images upload <file> --task <id>     # Attacher à une tâche
palnia images upload <file> --event <id>    # Attacher à un événement
palnia images download <id>           # Télécharger une image
palnia images delete <id>             # Supprimer
palnia images rename <id> "Nouveau nom"     # Renommer
palnia images quota                   # Voir le quota utilisé
```

## Identifiants courts

Les commandes qui prennent un `<id>` acceptent un **préfixe** de l'UUID. Par exemple si l'ID est `d02c3f4c-...`, vous pouvez taper :

```bash
palnia tasks done d02c
```

## Intégrations

### Claude Code

```bash
palnia init --claude-code
```

Génère `~/.claude/PALNIA.md` avec la documentation du CLI et ajoute la référence `@PALNIA.md` dans le `~/.claude/CLAUDE.md` global. Idempotent : peut être relancé sans risque de doublon.

### Mise à jour

```bash
palnia update    # Mettre à jour vers la dernière version
```

Le CLI vérifie automatiquement les mises à jour à chaque exécution et vous notifie si une nouvelle version est disponible.

## Configuration

| Variable | Description | Défaut |
|----------|-------------|--------|
| `PALNIA_API_URL` | URL de l'API Palnia | `https://palnia.newalfox.fr/api` |

## Stack

- **Rust** avec Tokio (async runtime)
- **clap** pour le parsing CLI
- **reqwest** pour les appels HTTP
- **colored** pour l'affichage terminal

## License

MIT
