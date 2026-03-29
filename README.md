# abtop

A monitor for AI coding agents.

Inspired by [htop](https://github.com/htop-dev/htop) and [btop](https://github.com/aristocratos/btop).

Currently supports **Claude Code** and **Codex CLI**.

## Install

```bash
curl -L https://raw.githubusercontent.com/graykode/abtop/main/abtopup/install | bash
```

### Homebrew (macOS / Linux)

```bash
brew install graykode/tap/abtop
```

### Cargo (crates.io)

```bash
cargo install abtop
```

### Download Binary

Pre-built binaries for macOS and Linux (x86_64 / aarch64) are available on the [GitHub Releases](https://github.com/graykode/abtop/releases) page.

```bash
# Example: macOS Apple Silicon
curl -LO https://github.com/graykode/abtop/releases/latest/download/abtop-aarch64-apple-darwin.tar.gz
tar xzf abtop-aarch64-apple-darwin.tar.gz
sudo mv abtop /usr/local/bin/
```

### Build from Source

```bash
git clone https://github.com/graykode/abtop.git
cd abtop
cargo install --path .
```

## Usage

```bash
abtop          # Launch TUI
abtop --once   # Print snapshot and exit (debug mode)
```

## Supported Agents

| Feature | Claude Code | Codex CLI |
|---------|:-----------:|:---------:|
| Session Discovery | ✅ | ✅ |
| Transcript Parsing | ✅ | ✅ |
| Token Tracking | ✅ | ✅ |
| Context Window % | ✅ | ✅ |
| Status Detection | ✅ | ✅ |
| Current Task | ✅ | ✅ |
| Subagents | ✅ | ❌ |
| Memory Status | ✅ | ❌ |
| Rate Limit | ✅ | ✅ |
| Git Status | ✅ | ✅ |
| Children / Ports | ✅ | ✅ |
| Done Detection | ✅ | ✅ |
| Cache Tokens | ✅ | ✅ |
| Initial Prompt | ❌ | ✅ |

## Key Bindings

| Key | Action |
|-----|--------|
| `↑`/`↓` or `k`/`j` | Select session |
| `Enter` | Jump to session terminal (tmux only) |
| `Tab` | Cycle focus between panels |
| `1`–`4` | Toggle panel visibility |
| `q` | Quit |
| `r` | Force refresh |

## Tech Stack

- **Rust** (2021 edition)
- **ratatui** + **crossterm** for TUI
- **tokio** for async runtime
- **serde** + **serde_json** for JSONL parsing

## Privacy

abtop reads local files only. No network calls, no API keys, no auth. Tool names and file paths are shown in the UI, but file contents and prompt text are never displayed.

## License

MIT
