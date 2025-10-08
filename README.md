# MCSrvMonitor Backend

A **high-accuracy**, **high-performance**, **minimalistic** Minecraft server status monitor.

## Features

- [x] Efficient asynchronous polling of Minecraft server status
- [x] Automatic storage to **SQLite**
- [x] Simple configuration and easy deployment
- [x] Both Java and Bedrock Edition server support
- [x] Multiple server monitoring support

## Quick Start

### Requirements

- **Rust** 1.70+
- **SQLite3**

### Build & Run

```bash
git clone https://github.com/CaSilicate-dev/MCSrvMonitor.git
cd MCSrvMonitor

# Edit config.json as needed
cargo run --release
```

### Configuration

- port: Port of API service
- addr: Address of API service
- length: Length of verbose information
- backend: Sampling backend configuration
    - dbfile: Database filename
    - interval: Sampling interval (sec)
- servers: Register your server here
    - label: Display name
    - name: Internal identifier
    - addr: Server address:port
    - type: Server type(1: Java Edition; 0: Bedrock Edition)

Example `config.json`:

```json
{
    "port": 8003,
    "addr": "0.0.0.0",
    "length": 250,
    "backend": {
        "dbfile": "history.db",
        "interval": 15
    },
    "servers": [
        {
            "label": "[Java] FTS427 Zn-Server",
            "name": "fts",
            "addr": "server.fts427.top",
            "type": 1
        },
        {
            "label": "[Bedrock] FTS427 Zn-Server",
            "name": "ftsb",
            "addr": "server.fts427.top:19132",
            "type": 0
        }
    ]
}
```