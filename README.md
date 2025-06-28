# Claw 🦀

A simple and efficient command-line task tracking tool written in Rust.

[![CI](https://github.com/yourusername/taskclaw/workflows/CI/badge.svg)](https://github.com/yourusername/taskclaw/actions)
[![Release](https://github.com/yourusername/taskclaw/workflows/Release/badge.svg)](https://github.com/yourusername/taskclaw/releases)
[![Security](https://github.com/yourusername/taskclaw/workflows/Security%20Audit/badge.svg)](https://github.com/yourusername/taskclaw/actions)

## Features

- ✅ **Simple task management**: Add, complete, and remove tasks with ease
- 📋 **Clean interface**: Intuitive command-line with status indicators (○ pending, ✓ completed)
- 🚀 **Zero dependencies**: Static binaries that work anywhere (~720KB)
- ⚡ **Lightning fast**: Rust performance with instant startup
- 🔧 **Shell completions**: Bash, Zsh, Fish, and PowerShell support
- 📚 **Documentation**: Man pages and comprehensive help
- ⚙️ **Configurable**: TOML-based configuration system
- 🔒 **Secure**: Memory-safe Rust with regular security audits
- 🌍 **Cross-platform**: Linux, macOS, Windows support

## Installation

### 🚀 Standalone Static Binaries (Recommended)

Download a single executable with **zero dependencies** - just download and run!

#### One-Command Install

```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/yourusername/taskclaw/main/scripts/download-binary.sh | bash

# Windows PowerShell
iwr -useb https://raw.githubusercontent.com/yourusername/taskclaw/main/scripts/download-binary.ps1 | iex
```

#### Direct Downloads

Click to download for your platform (no installation required):

| Platform | Download | Size | Dependencies |
|----------|----------|------|--------------|
| **Linux (x86_64)** | [`claw-linux-x86_64-static`](https://github.com/yourusername/taskclaw/releases/latest/download/claw-linux-x86_64-static) | ~720KB | None |
| **macOS (Intel)** | [`claw-macos-x86_64-static`](https://github.com/yourusername/taskclaw/releases/latest/download/claw-macos-x86_64-static) | ~720KB | System only |
| **macOS (Apple Silicon)** | [`claw-macos-arm64-static`](https://github.com/yourusername/taskclaw/releases/latest/download/claw-macos-arm64-static) | ~720KB | System only |
| **Windows (x86_64)** | [`claw-windows-x86_64-static.exe`](https://github.com/yourusername/taskclaw/releases/latest/download/claw-windows-x86_64-static.exe) | ~720KB | None |

> 💡 **Why static binaries?** They work everywhere, have no dependencies, are perfect for CI/CD, containers, and air-gapped systems.

### 📦 Package Managers

#### Homebrew (macOS/Linux)

```bash
brew install yourusername/tap/claw
```

#### Linux Packages

```bash
# Debian/Ubuntu (.deb)
wget https://github.com/yourusername/taskclaw/releases/latest/download/claw_0.1.0_amd64.deb
sudo dpkg -i claw_0.1.0_amd64.deb

# RHEL/CentOS/Fedora (.rpm)
wget https://github.com/yourusername/taskclaw/releases/latest/download/claw-0.1.0-1.x86_64.rpm
sudo rpm -i claw-0.1.0-1.x86_64.rpm

# AppImage (Universal Linux)
wget https://github.com/yourusername/taskclaw/releases/latest/download/claw-0.1.0-x86_64.AppImage
chmod +x claw-0.1.0-x86_64.AppImage
./claw-0.1.0-x86_64.AppImage
```

### 🛠 Full Installation with Completions

For the complete experience with shell completions and man pages:

```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/yourusername/taskclaw/main/scripts/install.sh | bash

# Windows PowerShell
iwr -useb https://raw.githubusercontent.com/yourusername/taskclaw/main/scripts/install.ps1 | iex
```

### 📥 Other Installation Methods

<details>
<summary>From Source</summary>

```bash
git clone https://github.com/yourusername/taskclaw.git
cd taskclaw
cargo install --path .
```
</details>

<details>
<summary>Cargo Install</summary>

```bash
cargo install taskclaw
```
</details>

<details>
<summary>From GitHub Releases</summary>

Visit the [releases page](https://github.com/yourusername/taskclaw/releases) for:
- Source archives (`.tar.gz`, `.zip`)
- Full packages with completions
- All platform binaries
- Release notes and changelog
</details>

## Quick Start

```bash
# Add your first task
claw add "Learn Rust"

# List all tasks  
claw list
# Output:
# Tasks:
#   ○ [0] Learn Rust

# Complete the task
claw complete 0

# Verify it's done
claw list  
# Output:
# Tasks:
#   ✓ [0] Learn Rust

# Remove completed task
claw remove 0

# Get help anytime
claw --help
```

## Usage

### Core Commands

| Command | Description | Example |
|---------|-------------|---------|
| `claw add <description>` | Add a new task | `claw add "Buy groceries"` |
| `claw list` | List all tasks | `claw list` |
| `claw complete <id>` | Mark task as complete | `claw complete 0` |
| `claw remove <id>` | Remove a task | `claw remove 1` |
| `claw --help` | Show help | `claw --help` |
| `claw --version` | Show version | `claw --version` |

### Shell Completions

Enable tab completion for your shell:

```bash
# Bash
claw completions bash > ~/.local/share/bash-completion/completions/claw

# Zsh  
claw completions zsh > ~/.local/share/zsh/site-functions/_claw

# Fish
claw completions fish > ~/.config/fish/completions/claw.fish

# PowerShell (Windows)
claw completions powershell >> $PROFILE
```

### Configuration

Claw stores settings in `~/.config/claw/config.toml` (Linux/macOS) or `%APPDATA%\claw\config.toml` (Windows).

**Default configuration:**
```toml
data_format = "json"
default_priority = "medium"  
show_completed = true
```

**Available options:**
- `data_format`: Storage format (`"json"`)
- `default_priority`: Default task priority (`"low"`, `"medium"`, `"high"`)
- `show_completed`: Whether to show completed tasks (`true`/`false`)

## Binary Information

### Static Compilation Details

Our binaries are statically compiled for maximum compatibility:

| Platform | Linking | Dependencies | Notes |
|----------|---------|--------------|-------|
| **Linux** | Full static (musl) | None | Works on any Linux distro |
| **macOS** | Static where possible | System libraries only | Universal compatibility |
| **Windows** | Static runtime | None | Self-contained executable |

### Performance

- **Binary size**: ~720KB (optimized with LTO)
- **Startup time**: <1ms 
- **Memory usage**: ~2MB
- **Dependencies**: Zero external libraries

### Security

- ✅ **Memory safe**: Written in Rust
- ✅ **Regular audits**: Automated security scanning
- ✅ **Minimal attack surface**: Static binaries, no network code
- ✅ **Supply chain security**: Reproducible builds

## Development

### Prerequisites

- **Rust**: 1.70 or later
- **Cargo**: Latest stable

### Building

```bash
# Clone repository
git clone https://github.com/yourusername/taskclaw.git
cd taskclaw

# Regular build
cargo build --release

# Static build (like releases)
cargo build --profile release-static

# Development build
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_add_task

# Test with coverage
cargo test --coverage
```

### Quality Assurance

```bash
# Linting
cargo clippy -- -D warnings

# Formatting  
cargo fmt --check

# Security audit
cargo audit

# Benchmarks
cargo bench
```

### Local Testing Scripts

```bash
# Test static compilation
./scripts/test-static.sh

# Verify static binary
./scripts/verify-static.sh

# Test package generation  
./scripts/test-packages.sh
```

## CI/CD & Release Process

### Automated Workflows

- **CI**: Tests on Linux, macOS, Windows with multiple Rust versions
- **Security**: Daily dependency audits and vulnerability scanning  
- **Benchmarks**: Performance regression detection
- **Releases**: Automated binary building and publishing

### Creating a Release

```bash
# Tag a new version
git tag v0.2.0
git push --tags

# This automatically:
# 1. Builds static binaries for all platforms
# 2. Creates GitHub release with binaries
# 3. Publishes to crates.io
# 4. Updates Homebrew formula
# 5. Generates packages (deb, rpm, AppImage)
```

## Contributing

We welcome contributions! Here's how to get started:

### Quick Contributing Guide

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/yourusername/taskclaw.git`
3. **Create** a feature branch: `git checkout -b feature/amazing-feature`
4. **Make** your changes and add tests
5. **Test** locally: `cargo test && cargo clippy`
6. **Commit**: `git commit -m 'Add amazing feature'`
7. **Push**: `git push origin feature/amazing-feature`
8. **Create** a Pull Request

### Development Guidelines

- **Write tests** for new functionality
- **Follow** the existing code style (run `cargo fmt`)
- **Pass** all CI checks (`cargo test`, `cargo clippy`)
- **Update** documentation as needed
- **Add** changelog entries for user-facing changes

## Support

- 📖 **Documentation**: Built-in help with `claw --help`
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/yourusername/taskclaw/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/yourusername/taskclaw/discussions)
- 🔒 **Security Issues**: See [SECURITY.md](SECURITY.md)

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Built with these amazing technologies:

- 🦀 **[Rust](https://www.rust-lang.org/)** - Systems programming language
- ⚙️ **[clap](https://github.com/clap-rs/clap)** - Command line argument parsing  
- 📄 **[toml](https://github.com/toml-rs/toml)** - Configuration format
- 📁 **[dirs](https://github.com/dirs-dev/dirs-rs)** - Cross-platform paths
- 🔧 **[serde](https://github.com/serde-rs/serde)** - Serialization framework

---

<div align="center">

**[⬆ Back to Top](#claw-)**

Made with ❤️ and 🦀 by the TaskClaw contributors

</div>