#!/usr/bin/env node

const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');

const platform = os.platform();
const binDir = path.join(os.homedir(), '.palnia', 'bin');
const binName = platform === 'win32' ? 'palnia.exe' : 'palnia';
const binPath = path.join(binDir, binName);

// Utilise spawnSync pour éviter le warning de sécurité avec spawn + shell
const result = spawnSync(binPath, process.argv.slice(2), {
  stdio: 'inherit',
});

if (result.error) {
  if (result.error.code === 'ENOENT') {
    console.error(`Erreur: binaire non trouvé à ${binPath}`);
    console.error('Réessayez: npm install -g @oalacea/palnia-cli');
  } else {
    console.error('Erreur:', result.error.message);
  }
  process.exit(1);
}

process.exit(result.status ?? 0);
