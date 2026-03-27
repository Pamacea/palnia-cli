#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const os = require('os');

const installDir = path.join(os.homedir(), '.palnia');

// Remove installation directory
if (fs.existsSync(installDir)) {
  fs.rmSync(installDir, { recursive: true, force: true });
  console.log('✓ Palnia CLI uninstalled');
} else {
  console.log('Palnia CLI was not installed');
}
