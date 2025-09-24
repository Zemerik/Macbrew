# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ----------------- |
| 1.0.1   | :white_check_mark: |

Older versions may not receive security updates.

## Reporting a Vulnerability

If you discover a security vulnerability, **please do not publicly disclose it** until it has been reviewed and addressed. To report a vulnerability:

- Use [GitHub Security Advisories](https://github.com/Zemerik/Macbrew/security/advisories) for confidential reporting.
- Or email: zemeriky@gmail.com

We aim to respond to security reports within 7 days.

## Scope and Security Considerations

- This project is an open-source terminal emulator for Linux and Windows.
- Command history is stored in a file named `.terminal_emulator_history` in the user's home directory. This file contains the commands run, timestamps, and exit codes. **If users run commands containing secrets, those will be stored in history.**
- The emulator can execute system commands (e.g., `curl`, `wget`, `ping`) and install/run plugins written in Python, Bash, or Rust. **Plugins and scripts may execute arbitrary code. Only install plugins from trusted sources.**
- The project uses third-party dependencies (see `Cargo.toml` and `package.json`). Standard supply chain vigilance is recommended.

## Best Practices for Users

- Do not run untrusted plugins or scripts.
- Be aware that sensitive data in commands will be stored in your history file.
- Keep your installation up to date with the latest releases.

## Best Practices for Contributors

- Avoid introducing dependencies with known vulnerabilities.
- Review code for potential injection, privilege escalation, or unsafe file operations.
- Disclose vulnerabilities responsibly as described above.

## Security Update Policy

- Security fixes will be released as soon as possible after a vulnerability is confirmed and patched.
- Users are encouraged to update to the latest version promptly.

---

For any questions or clarifications, contact: zemeriky@gmail.com 