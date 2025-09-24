---
layout: ../../layouts/Layout.astro
title: Command Reference - Macbrew Documentation
description: Complete reference of all available commands in Macbrew
---

# Command Reference

Complete reference of all available commands in Macbrew terminal emulator.

## File Operations

### Basic File Commands

| Command | Description | Usage |
|---------|-------------|-------|
| `ls` | List directory contents | `ls [options] [path]` |
| `cd` | Change directory | `cd [directory]` |
| `pwd` | Print working directory | `pwd` |
| `cat` | Concatenate and display files | `cat [file]` |
| `echo` | Display a line of text | `echo [text]` |
| `mkdir` | Create directories | `mkdir [directory]` |
| `rm` | Remove files/directories | `rm [file/directory]` |
| `cp` | Copy files/directories | `cp [source] [destination]` |
| `mv` | Move/rename files/directories | `mv [source] [destination]` |

### Advanced File Operations

| Command | Description | Usage |
|---------|-------------|-------|
| `find` | Find files by criteria | `find [path] [criteria]` |
| `grep` | Search text patterns | `grep [pattern] [file]` |
| `chmod` | Change file permissions | `chmod [mode] [file]` |
| `chown` | Change file owner | `chown [user] [file]` |
| `chgrp` | Change file group | `chgrp [group] [file]` |
| `ln` | Create links | `ln [source] [link]` |
| `touch` | Create empty files | `touch [file]` |
| `file` | Determine file type | `file [file]` |
| `stat` | Display file status | `stat [file]` |

### Text Processing

| Command | Description | Usage |
|---------|-------------|-------|
| `head` | Display first lines | `head [options] [file]` |
| `tail` | Display last lines | `tail [options] [file]` |
| `sort` | Sort lines | `sort [options] [file]` |
| `uniq` | Remove duplicate lines | `uniq [options] [file]` |
| `cut` | Extract sections | `cut [options] [file]` |
| `paste` | Merge lines | `paste [options] [file1] [file2]` |
| `tr` | Translate characters | `tr [options] [set1] [set2]` |
| `sed` | Stream editor | `sed [options] [script] [file]` |
| `awk` | Pattern scanning | `awk [options] [program] [file]` |
| `wc` | Word count | `wc [options] [file]` |

## System Administration

### Process Management

| Command | Description | Usage |
|---------|-------------|-------|
| `ps` | Process status | `ps [options]` |
| `top` | Process monitor | `top [options]` |
| `htop` | Interactive process viewer | `htop [options]` |
| `kill` | Terminate processes | `kill [signal] [pid]` |
| `pkill` | Kill by name | `pkill [options] [pattern]` |
| `nohup` | Run immune to hangups | `nohup [command]` |
| `nice` | Run with priority | `nice [options] [command]` |
| `renice` | Change priority | `renice [priority] [pid]` |

### System Services

| Command | Description | Usage |
|---------|-------------|-------|
| `systemctl` | Control systemd | `systemctl [command] [service]` |
| `service` | Control services | `service [service] [command]` |
| `init` | Process control | `init [runlevel]` |
| `systemd-analyze` | Analyze boot | `systemd-analyze [options]` |

### User Management

| Command | Description | Usage |
|---------|-------------|-------|
| `useradd` | Add user | `useradd [options] [username]` |
| `userdel` | Delete user | `userdel [options] [username]` |
| `usermod` | Modify user | `usermod [options] [username]` |
| `passwd` | Change password | `passwd [username]` |
| `groupadd` | Add group | `groupadd [options] [groupname]` |
| `groupdel` | Delete group | `groupdel [groupname]` |
| `id` | User identity | `id [username]` |
| `who` | Show logged users | `who [options]` |
| `w` | Show user activity | `w [options]` |

### Disk Management

| Command | Description | Usage |
|---------|-------------|-------|
| `df` | Disk space | `df [options] [path]` |
| `du` | Directory usage | `du [options] [path]` |
| `mount` | Mount filesystem | `mount [options] [device] [path]` |
| `umount` | Unmount filesystem | `umount [options] [path]` |
| `fdisk` | Partition table | `fdisk [options] [device]` |
| `parted` | Partition editor | `parted [options] [device]` |
| `mkfs` | Make filesystem | `mkfs [options] [device]` |
| `fsck` | Check filesystem | `fsck [options] [device]` |

## Networking

### Network Diagnostics

| Command | Description | Usage |
|---------|-------------|-------|
| `ping` | Test connectivity | `ping [options] [host]` |
| `ping6` | IPv6 ping | `ping6 [options] [host]` |
| `traceroute` | Trace route | `traceroute [options] [host]` |
| `mtr` | Network diagnostic | `mtr [options] [host]` |
| `nslookup` | DNS lookup | `nslookup [host]` |
| `dig` | DNS query | `dig [options] [host]` |
| `host` | DNS lookup | `host [options] [host]` |

### Network Information

| Command | Description | Usage |
|---------|-------------|-------|
| `ifconfig` | Interface config | `ifconfig [interface]` |
| `ip` | IP utility | `ip [object] [command]` |
| `netstat` | Network statistics | `netstat [options]` |
| `ss` | Socket statistics | `ss [options]` |
| `route` | Routing table | `route [options]` |
| `arp` | ARP table | `arp [options]` |

### Network Tools

| Command | Description | Usage |
|---------|-------------|-------|
| `curl` | Transfer data | `curl [options] [url]` |
| `wget` | Retrieve files | `wget [options] [url]` |
| `ssh` | Secure shell | `ssh [options] [user@]host` |
| `scp` | Secure copy | `scp [options] [source] [destination]` |
| `rsync` | Remote sync | `rsync [options] [source] [destination]` |
| `telnet` | Telnet client | `telnet [host] [port]` |
| `nc` | Netcat | `nc [options] [host] [port]` |
| `socat` | Multipurpose relay | `socat [options] [address1] [address2]` |

### Network Analysis

| Command | Description | Usage |
|---------|-------------|-------|
| `nmap` | Network scanner | `nmap [options] [target]` |
| `tcpdump` | Packet analyzer | `tcpdump [options] [expression]` |
| `wireshark` | Network protocol analyzer | `wireshark [options]` |
| `iftop` | Network monitor | `iftop [options]` |
| `nethogs` | Network usage | `nethogs [options]` |
| `iotop` | I/O monitor | `iotop [options]` |

## Security & Cryptography

### Hashing

| Command | Description | Usage |
|---------|-------------|-------|
| `md5sum` | MD5 hash | `md5sum [file]` |
| `sha1sum` | SHA1 hash | `sha1sum [file]` |
| `sha256sum` | SHA256 hash | `sha256sum [file]` |
| `sha512sum` | SHA512 hash | `sha512sum [file]` |
| `cksum` | Checksum | `cksum [file]` |

### Encryption

| Command | Description | Usage |
|---------|-------------|-------|
| `openssl` | SSL/TLS toolkit | `openssl [command] [options]` |
| `gpg` | GNU Privacy Guard | `gpg [options] [command]` |
| `base64` | Base64 encoding | `base64 [options] [file]` |
| `hexdump` | Hex dump | `hexdump [options] [file]` |
| `xxd` | Hex dump | `xxd [options] [file]` |

### SSH Management

| Command | Description | Usage |
|---------|-------------|-------|
| `ssh-keygen` | Generate SSH keys | `ssh-keygen [options]` |
| `ssh-copy-id` | Copy SSH key | `ssh-copy-id [options] [user@]host` |
| `ssh-add` | Add SSH key | `ssh-add [options] [key]` |
| `ssh-agent` | SSH agent | `ssh-agent [options]` |

## Package Management

### Homebrew

| Command | Description | Usage |
|---------|-------------|-------|
| `brew` | Homebrew package manager | `brew [command] [package]` |
| `brew install` | Install package | `brew install [package]` |
| `brew uninstall` | Remove package | `brew uninstall [package]` |
| `brew search` | Search packages | `brew search [query]` |
| `brew update` | Update Homebrew | `brew update` |
| `brew upgrade` | Upgrade packages | `brew upgrade [package]` |
| `brew list` | List packages | `brew list` |
| `brew info` | Package info | `brew info [package]` |

### Python Package Management

| Command | Description | Usage |
|---------|-------------|-------|
| `pip` | Python package installer | `pip [command] [package]` |
| `pip install` | Install package | `pip install [package]` |
| `pip uninstall` | Remove package | `pip uninstall [package]` |
| `pip list` | List packages | `pip list` |
| `pip freeze` | Freeze requirements | `pip freeze` |
| `conda` | Conda package manager | `conda [command] [package]` |
| `conda install` | Install package | `conda install [package]` |
| `conda create` | Create environment | `conda create -n [env] [packages]` |

### Node.js Package Management

| Command | Description | Usage |
|---------|-------------|-------|
| `npm` | Node package manager | `npm [command] [package]` |
| `npm install` | Install package | `npm install [package]` |
| `npm uninstall` | Remove package | `npm uninstall [package]` |
| `npm list` | List packages | `npm list` |
| `npm update` | Update packages | `npm update` |
| `yarn` | Yarn package manager | `yarn [command] [package]` |
| `yarn add` | Add package | `yarn add [package]` |
| `yarn remove` | Remove package | `yarn remove [package]` |

## DevOps & Cloud

### Container Management

| Command | Description | Usage |
|---------|-------------|-------|
| `docker` | Docker container platform | `docker [command] [options]` |
| `docker run` | Run container | `docker run [options] [image]` |
| `docker ps` | List containers | `docker ps [options]` |
| `docker images` | List images | `docker images [options]` |
| `docker build` | Build image | `docker build [options] [path]` |
| `kubectl` | Kubernetes CLI | `kubectl [command] [resource]` |
| `kubectl get` | Get resources | `kubectl get [resource]` |
| `kubectl apply` | Apply configuration | `kubectl apply -f [file]` |
| `helm` | Kubernetes package manager | `helm [command] [release]` |

### Infrastructure as Code

| Command | Description | Usage |
|---------|-------------|-------|
| `terraform` | Infrastructure automation | `terraform [command]` |
| `terraform init` | Initialize | `terraform init` |
| `terraform plan` | Plan changes | `terraform plan` |
| `terraform apply` | Apply changes | `terraform apply` |
| `ansible` | Automation tool | `ansible [options] [pattern]` |
| `ansible-playbook` | Run playbook | `ansible-playbook [playbook]` |
| `puppet` | Configuration management | `puppet [command] [options]` |
| `chef` | Configuration management | `chef [command] [options]` |

## Text Editors

### Command Line Editors

| Command | Description | Usage |
|---------|-------------|-------|
| `vim` | Vi improved | `vim [file]` |
| `nano` | Nano editor | `nano [file]` |
| `emacs` | Emacs editor | `emacs [file]` |
| `ed` | Line editor | `ed [file]` |

### Pagers

| Command | Description | Usage |
|---------|-------------|-------|
| `less` | File pager | `less [options] [file]` |
| `more` | File pager | `more [options] [file]` |
| `most` | File pager | `most [options] [file]` |

## Compression & Archives

### File Compression

| Command | Description | Usage |
|---------|-------------|-------|
| `gzip` | Compress files | `gzip [options] [file]` |
| `gunzip` | Decompress files | `gunzip [options] [file]` |
| `bzip2` | Compress files | `bzip2 [options] [file]` |
| `bunzip2` | Decompress files | `bunzip2 [options] [file]` |
| `xz` | Compress files | `xz [options] [file]` |
| `unxz` | Decompress files | `unxz [options] [file]` |

### Archive Management

| Command | Description | Usage |
|---------|-------------|-------|
| `tar` | Tape archive | `tar [options] [file] [files]` |
| `zip` | Create archive | `zip [options] [archive] [files]` |
| `unzip` | Extract archive | `unzip [options] [archive]` |
| `7z` | 7-Zip archiver | `7z [command] [archive] [files]` |
| `rar` | RAR archiver | `rar [command] [archive] [files]` |
| `unrar` | Extract RAR | `unrar [command] [archive]` |

## Logging & Monitoring

### System Logs

| Command | Description | Usage |
|---------|-------------|-------|
| `dmesg` | Kernel messages | `dmesg [options]` |
| `journalctl` | Systemd journal | `journalctl [options]` |
| `logwatch` | Log analysis | `logwatch [options]` |
| `rsyslog` | System logging | `rsyslog [options]` |
| `logrotate` | Log rotation | `logrotate [options] [config]` |

## Terminal Multiplexing

### Session Management

| Command | Description | Usage |
|---------|-------------|-------|
| `screen` | Terminal multiplexer | `screen [options] [command]` |
| `tmux` | Terminal multiplexer | `tmux [command] [options]` |
| `byobu` | Terminal multiplexer | `byobu [command] [options]` |

## Macbrew Specific Commands

### Plugin Management

| Command | Description | Usage |
|---------|-------------|-------|
| `plugins` | Plugin management | `plugins [command]` |
| `plugins list` | List plugins | `plugins list` |
| `install-plugin` | Install plugin | `install-plugin [name]` |
| `uninstall-plugin` | Remove plugin | `uninstall-plugin [name]` |
| `enable-plugin` | Enable plugin | `enable-plugin [name]` |
| `disable-plugin` | Disable plugin | `disable-plugin [name]` |

### Configuration

| Command | Description | Usage |
|---------|-------------|-------|
| `config` | Configuration management | `config [command] [key] [value]` |
| `config get` | Get setting | `config get [key]` |
| `config set` | Set setting | `config set [key] [value]` |
| `config list` | List settings | `config list` |
| `config reset` | Reset to defaults | `config reset` |

### Built-in Utilities

| Command | Description | Usage |
|---------|-------------|-------|
| `alias` | Manage aliases | `alias [name='value']` |
| `unalias` | Remove alias | `unalias [name]` |
| `export` | Set environment | `export [name=value]` |
| `unset` | Unset variable | `unset [name]` |
| `env` | Show environment | `env` |
| `history` | Command history | `history [options]` |
| `clear` | Clear screen | `clear` |
| `exit` | Exit shell | `exit [code]` |

## Getting Help

### Help Commands

| Command | Description | Usage |
|---------|-------------|-------|
| `help` | Built-in help | `help [command]` |
| `man` | Manual pages | `man [section] [topic]` |
| `info` | Info pages | `info [topic]` |
| `apropos` | Search man pages | `apropos [keyword]` |
| `whatis` | One-line description | `whatis [command]` |

## Examples

### Common Workflows

**File Management:**
```bash
# Create and navigate to directory
mkdir myproject && cd myproject

# Create files
touch README.md script.py

# List with details
ls -la

# View file content
cat README.md
```

**Process Management:**
```bash
# Check running processes
ps aux | grep python

# Monitor system resources
top

# Kill process by PID
kill -9 12345
```

**Network Operations:**
```bash
# Test connectivity
ping google.com

# Download file
curl -O https://example.com/file.zip

# SSH to server
ssh user@server.com
```

**Package Management:**
```bash
# Install with Homebrew
brew install python

# Install Python package
pip install requests

# Install Node.js package
npm install express
```

## Next Steps

- [Plugin System](/docs/plugins) - Extend functionality with plugins
- [Configuration](/docs/configuration) - Customize your experience
- [API Reference](/docs/api) - Programmatic access to Macbrew
- [Troubleshooting](/docs/troubleshooting) - Common issues and solutions 