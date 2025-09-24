#!/usr/bin/env node

/**
 * JavaScript wrapper for the TypeScript install script
 * This ensures compatibility and calls the compiled TypeScript version
 */

const { spawn } = require('child_process');
const path = require('path');

function runTypeScriptInstall() {
  const scriptPath = path.join(__dirname, '..', 'dist', 'scripts', 'install.js');
  
  // Check if compiled version exists
  const fs = require('fs');
  if (fs.existsSync(scriptPath)) {
    // Run the compiled TypeScript version
    require(scriptPath);
  } else {
    // Fallback: run with ts-node if available
    try {
      const tsNode = require('ts-node');
      const installScript = path.join(__dirname, '..', 'src', 'scripts', 'install.ts');
      require(installScript);
    } catch (error) {
      console.error('❌ Error: TypeScript version not compiled and ts-node not available');
      console.error('Please run: npm run build');
      process.exit(1);
    }
  }
}

// Run the installation
runTypeScriptInstall(); 