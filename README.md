English | [简体中文](docs/README.zh-CN.md)

# MCSrvMonitor Backend

A high-accuracy, high-performance, minimalistic Minecraft server status monitor.

## Features

- [x] Efficient asynchronous polling of Minecraft server status
- [x] Automatic storage to SQLite database
- [x] Simple configuration and easy deployment
- [ ] Both Java and Bedrock Edition server support
- [x] Multiple server monitoring support
## Quick Start

### Requirements

- Rust 1.70+
- SQLite3

### Build & Run

```bash
git clone https://github.com/CaSilicate-dev/MCSrvMonitor.git
cd MCSrvMonitor
# Edit config.json as needed
cargo run --release
```

### Configuration

Example `config.json`:

```json
{
    "port": 18650,
    "addr": "0.0.0.0",
    "length": 250,
    "backend": {
        "dbfile": "history.db",
        "interval": 1
    },
    "servers": [
        {
            "name": "fts",
            "addr": "server.fts427.top"
        },
        {
            "name": "hypixel",
            "addr": "mc.hypixel.net"
        }
    ]
}

```

[Configuration Explained docs/config.md](docs/config.md)
