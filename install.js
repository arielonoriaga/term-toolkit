const os = require('os');
const fs = require('fs');
const path = require('path');

const platform = os.platform();
const binDir = path.join(__dirname, 'dist');
const binaryName = 'term-toolkit';
const binaryPath = path.join(binDir, binaryName);

try {
  fs.chmodSync(binaryPath, 0o755);
  console.log(`Binary installed successfully: ${binaryPath}`);
} catch (error) {
  console.error(`Error setting permissions for binary: ${error.message}`);
  process.exit(1);
}
