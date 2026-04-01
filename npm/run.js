#!/usr/bin/env node

const os = require('os');
const path = require('path');
const { spawn } = require('child_process');

const platform = os.platform();
const binDir = path.join(os.homedir(), '.palnia', 'bin');
const binName = platform === 'win32' ? 'palnia.exe' : 'palnia';
const binPath = path.join(binDir, binName);

const options = {
  stdio: 'inherit',
};

// Sur Windows, utilise shell seulement si nécessaire pour .exe
if (platform === 'win32') {
  options.shell = false; // Pas de shell = plus sécurisé
}

const child = spawn(binPath, process.argv.slice(2), options);

child.on('error', (err) => {
  if (err.code === 'ENOENT') {
    console.error(`Erreur: binaire non trouvé à ${binPath}`);
    console.error('Réessayez: npm install -g @oalacea/palnia-cli');
  } else {
    console.error('Erreur:', err.message);
  }
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code ?? 0);
});
