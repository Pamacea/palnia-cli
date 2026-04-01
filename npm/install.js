#!/usr/bin/env node

const https = require('https');
const http = require('http');
const fs = require('fs');
const path = require('path');
const os = require('os');

const PACKAGE_NAME = '@palnia/cli';
const VERSION = '0.2.3';
const GITHUB_REPO = 'Pamacea/palnia-cli';

// Detect platform
const platform = os.platform();
const arch = os.arch();

let binaryName, assetName;

if (platform === 'win32') {
  binaryName = 'palnia.exe';
  assetName = `palnia-${VERSION}-x86_64-pc-windows-msvc.exe`;
} else if (platform === 'darwin') {
  binaryName = 'palnia';
  if (arch === 'arm64') {
    assetName = `palnia-${VERSION}-aarch64-apple-darwin`;
  } else {
    assetName = `palnia-${VERSION}-x86_64-apple-darwin`;
  }
} else if (platform === 'linux') {
  binaryName = 'palnia';
  if (arch === 'arm64') {
    assetName = `palnia-${VERSION}-aarch64-unknown-linux-gnu`;
  } else {
    assetName = `palnia-${VERSION}-x86_64-unknown-linux-gnu`;
  }
} else {
  console.error(`Platform ${platform} not supported`);
  process.exit(1);
}

// Installation directory
const installDir = path.join(os.homedir(), '.palnia', 'bin');
const binPath = path.join(installDir, binaryName);

console.log(`Installing ${PACKAGE_NAME} v${VERSION}...`);
console.log(`  Platform: ${platform}`);
console.log(`  Asset: ${assetName}`);

// Create directory
if (!fs.existsSync(installDir)) {
  fs.mkdirSync(installDir, { recursive: true });
}

// Download URL
const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${assetName}`;

// Download function
function download(url, dest) {
  return new Promise((resolve, reject) => {
    const protocol = url.startsWith('https') ? https : http;

    console.log(`  Downloading from ${downloadUrl}...`);

    const file = fs.createWriteStream(dest);

    protocol.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        download(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }

      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      }

      const totalSize = parseInt(response.headers['content-length'], 10);
      let downloadedSize = 0;

      response.pipe(file);

      response.on('data', (chunk) => {
        downloadedSize += chunk.length;
        if (totalSize) {
          const percent = Math.floor((downloadedSize / totalSize) * 100);
          process.stdout.write(`\r  Progress: ${percent}%`);
        }
      });

      file.on('finish', () => {
        process.stdout.write('\n');
        resolve();
      });

      file.on('error', (err) => {
        fs.unlink(dest, () => {});
        reject(err);
      });
    }).on('error', (err) => {
      fs.unlink(dest, () => {});
      reject(err);
    });
  });
}

// Make executable (Unix)
function makeExecutable(filePath) {
  if (platform !== 'win32') {
    try {
      fs.chmodSync(filePath, 0o755);
    } catch (err) {
      console.warn(`  Warning: Could not make executable: ${err.message}`);
    }
  }
}

// Download binary
download(downloadUrl, binPath)
  .then(() => {
    makeExecutable(binPath);
    console.log(`  ✓ Installed to ${binPath}`);
    console.log(`\nTo get started:\n  palnia login`);
  })
  .catch((err) => {
    console.error(`  ✗ Installation failed: ${err.message}`);
    process.exit(1);
  });
