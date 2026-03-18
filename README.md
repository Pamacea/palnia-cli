# Plania CLI

CLI Rust pour [Plania](https://plania.newalfox.fr), l'app de productivité tout-en-un.

## Installation

```bash
cargo install --path .
```

## Authentification

Le CLI utilise des **tokens API** générés depuis l'app Plania (Paramètres > Tokens API).

```bash
plania login
# Coller votre token API (plt_...)
# URL par défaut : http://localhost:3001/api

plania whoami    # Vérifier la connexion
plania logout    # Se déconnecter
```

Configuration stockée dans `~/.plania/config.toml`. L'URL de l'API est configurable via `--url` ou la variable d'env `PLANIA_API_URL`.

## Commandes

### Tâches

```bash
plania tasks                          # Lister les tâches actives (todo/doing)
plania tasks all                      # Lister toutes les tâches (inclut done)
plania tasks add "Titre"              # Créer une tâche
plania tasks add "Titre" \
  -c professional \                   # Catégorie: spiritual, personal, professional
  -p urgent \                         # Priorité: urgent, normal, low
  --due 2026-03-20 \                  # Date d'échéance
  -n "Notes ici" \                    # Notes
  -t cli,rust \                       # Tags (séparés par virgule)
  -s "Sous-tâche 1,Sous-tâche 2"     # Sous-tâches

plania tasks doing <id>               # Marquer en cours
plania tasks done <id>                # Marquer terminée
plania tasks delete <id>              # Supprimer
plania tasks subtask <id> "Titre"     # Ajouter une sous-tâche
```

### Événements

```bash
plania events                         # Événements du jour
plania events week                    # Événements de la semaine
plania events add "Titre" \
  --date 2026-03-20 \                 # Date (requis)
  --start 14:00 \                     # Heure de début (défaut: 09:00)
  --end 15:30 \                       # Heure de fin (défaut: 10:00)
  -c professional \                   # Catégorie
  -d "Description" \                  # Description
  -n "Notes" \                        # Notes
  -t meeting,demo \                   # Tags
  --all-day                           # Événement journée entière

plania events delete <id>             # Supprimer
```

### Habitudes

```bash
plania habits                         # Lister + statut du jour
plania habits add "Titre"             # Créer une habitude
plania habits add "Titre" \
  -c spiritual \                      # Catégorie
  -f weekly                           # Fréquence: daily, weekly

plania habits toggle <id>             # Cocher/décocher aujourd'hui
plania habits toggle <id> --date 2026-03-17  # Cocher une date spécifique
plania habits delete <id>             # Supprimer
```

### Agenda

```bash
plania agenda                         # Vue combinée du jour (événements + tâches)
plania agenda week                    # Vue de la semaine
```

## Identifiants courts

Les commandes qui prennent un `<id>` acceptent un **préfixe** de l'UUID. Par exemple si l'ID est `d02c3f4c-...`, vous pouvez taper :

```bash
plania tasks done d02c
```

## Configuration

| Variable | Description | Défaut |
|----------|-------------|--------|
| `PLANIA_API_URL` | URL de l'API Plania | `http://localhost:3001/api` |

## Stack

- **Rust** avec Tokio (async runtime)
- **clap** pour le parsing CLI
- **reqwest** pour les appels HTTP
- **colored** pour l'affichage terminal

## License

MIT
