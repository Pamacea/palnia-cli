# @palnia/cli

> npm i -g @palnia/cli

CLI Rust pour [Palnia](https://palnia.newalfox.fr), l'app de productivité tout-en-un.

## Installation

```bash
npm install -g @palnia/cli
```

## Usage

Une fois installé, la commande `palnia` est disponible :

```bash
palnia login
palnia tasks add "Ma tâche"
palnia events week
palnia images quota
```

## Commandes principales

- `palnia login` / `logout` / `whoami` - Authentification
- `palnia tasks` - Gestion des tâches
- `palnia events` - Gestion des événements
- `palnia habits` - Gestion des habitudes
- `palnia agenda` - Vue combinée
- `palnia images` - Gestion des images

## Documentation

Pour la documentation complète, voir le [README principal](https://github.com/Pamacea/palnia-cli) du projet.

## Installation système

Le package npm installe automatiquement le binaire adapté à votre OS :
- **Windows** : `.exe` statique
- **macOS** : Universal binary (Intel + ARM)
- **Linux** : binaire x86_64

## Développement

Le code source est disponible sur [GitHub](https://github.com/Pamacea/palnia-cli).

## License

MIT
