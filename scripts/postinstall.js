#!/usr/bin/env node

/**
 * JavaScript wrapper for the TypeScript postinstall script
 * This ensures compatibility and calls the compiled TypeScript version
 */

const { spawn } = require('child_process');
const path = require('path');

function runTypeScriptPostinstall() {
  const scriptPath = path.join(__dirname, '..', 'dist', 'scripts', 'postinstall.js');
  
  // Check if compiled version exists
  const fs = require('fs');
  if (fs.existsSync(scriptPath)) {
    // Run the compiled TypeScript version
    require(scriptPath);
  } else {
    // Fallback: run with ts-node if available
    try {
      const tsNode = require('ts-node');
      const postinstallScript = path.join(__dirname, '..', 'src', 'scripts', 'postinstall.ts');
      require(postinstallScript);
    } catch (error) {
      console.error('❌ Error: TypeScript version not compiled and ts-node not available');
      console.error('Please run: npm run build');
      process.exit(1);
    }
  }
}

// Run the postinstall
runTypeScriptPostinstall(); 