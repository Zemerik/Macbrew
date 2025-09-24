# 📦 Publishing Macbrew to NPM

This guide explains how to publish the Macbrew to the npm registry.

## 🚀 Prerequisites

1. **NPM Account**: Create an account at [npmjs.com](https://www.npmjs.com/)
2. **NPM CLI**: Install npm CLI tools
3. **GitHub Account**: For repository hosting
4. **Domain**: Optional - for custom website

## 📋 Pre-Publishing Checklist

### 1. Update Version
```bash
# Update version in package.json
npm version patch  # 1.0.0 -> 1.0.1
npm version minor  # 1.0.0 -> 1.1.0
npm version major  # 1.0.0 -> 2.0.0
```

### 2. Test Build
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Test the binary
./target/release/terminal-emulator --help
```

### 3. Test Installation
```bash
# Test local installation
npm pack
npm install -g macbrew-1.0.0.tgz

# Test the package
macbrew --help
```

### 4. Update Documentation
- [ ] Update README.md with latest features
- [ ] Update version numbers
- [ ] Test all code examples
- [ ] Update changelog

## 🔐 NPM Authentication

### 1. Login to NPM
```bash
npm login
# Enter your username, password, and email
```

### 2. Verify Authentication
```bash
npm whoami
```

### 3. Check Package Name Availability
```bash
npm search macbrew
```

## 📤 Publishing Steps

### 1. Prepare for Publishing
```bash
# Clean build artifacts
cargo clean
cargo build --release

# Test the package locally
npm pack
```

### 2. Publish to NPM
```bash
# Publish to npm registry
npm publish

# Or publish with specific tag
npm publish --tag beta
```

### 3. Verify Publication
```bash
# Check package on npm
npm view macbrew

# Install and test
npm install -g macbrew
macbrew --help
```

## 🏷️ Version Management

### Semantic Versioning
- **Patch** (1.0.0 → 1.0.1): Bug fixes
- **Minor** (1.0.0 → 1.1.0): New features, backward compatible
- **Major** (1.0.0 → 2.0.0): Breaking changes

### Release Tags
```bash
# Latest stable
npm publish

# Beta release
npm publish --tag beta

# Alpha release
npm publish --tag alpha
```

## 🔄 CI/CD Setup

### GitHub Actions Workflow
Create `.github/workflows/publish.yml`:

```yaml
name: Publish to NPM

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install dependencies
        run: npm ci
      
      - name: Build Rust binary
        run: cargo build --release
      
      - name: Publish to NPM
        run: npm publish
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

## 📊 Post-Publishing

### 1. Monitor Installation
```bash
# Check download statistics
npm stats macbrew

# Monitor for issues
npm bugs macbrew
```

### 2. Update Documentation
- [ ] Update GitHub repository
- [ ] Update website (if applicable)
- [ ] Update social media
- [ ] Respond to issues/feedback

### 3. Maintenance
- [ ] Monitor for security vulnerabilities
- [ ] Update dependencies regularly
- [ ] Address user feedback
- [ ] Plan next release

## 🛠️ Troubleshooting

### Common Issues

#### Package Name Already Taken
```bash
# Check if name is available
npm search macbrew

# Consider alternative names:
# - mac-brew
# - macos-brew
# - homebrew-cli
# - advanced-brew
```

#### Build Failures
```bash
# Check Rust toolchain
rustup show

# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

#### Permission Issues
```bash
# Check npm permissions
npm whoami

# Fix permissions
npm login
```

#### Binary Not Found
```bash
# Ensure binary is built
cargo build --release

# Check binary exists
ls -la target/release/terminal-emulator
```

## 📈 Marketing

### 1. Social Media
- [ ] Twitter announcement
- [ ] Reddit posts (r/rust, r/nodejs, r/commandline)
- [ ] Hacker News submission
- [ ] Dev.to article

### 2. Documentation
- [ ] Comprehensive README
- [ ] API documentation
- [ ] Examples and tutorials
- [ ] Video demonstrations

### 3. Community
- [ ] GitHub discussions
- [ ] Discord/Slack channels
- [ ] Stack Overflow presence
- [ ] Conference talks

## 🔗 Useful Links

- [NPM Publishing Guide](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry)
- [Package.json Reference](https://docs.npmjs.com/cli/v8/configuring-npm/package-json)
- [Semantic Versioning](https://semver.org/)
- [NPM Security](https://docs.npmjs.com/about-audit-reports)

---

**Happy Publishing! 🚀** 