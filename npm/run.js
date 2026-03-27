#!/usr/bin/env node

const path = require('path');
const os = require('os');
const { spawn } = require('child_process');

const platform = os.platform();
const binaryName = platform === 'win32' ? 'palnia.exe' : 'palnia';
const binPath = path.join(os.homedir(), '.palnia', 'bin', binaryName);

// Spawn the binary with the same args
const child = spawn(binPath, process.argv.slice(2), {
  stdio: 'inherit',
});

child.on('error', (err) => {
  console.error(`Error running palnia: ${err.message}`);
  console.error(`Please run: npm install -g @palnia/cli`);
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code ?? 0);
});
