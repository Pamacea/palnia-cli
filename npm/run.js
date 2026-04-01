#!/usr/bin/env node

const os = require('os');
const path = require('path');
const { spawn } = require('child_process');

const platform = os.platform();
let binName, binDir;

// Same directory as install.js uses
binDir = path.join(os.homedir(), '.palnia', 'bin');
binName = platform === 'win32' ? 'palnia.exe' : 'palnia';

const binPath = path.join(binDir, binName);

// Spawn the binary with all arguments
const child = spawn(binPath, process.argv.slice(2), {
  stdio: 'inherit',
  shell: platform === 'win32'
});

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
