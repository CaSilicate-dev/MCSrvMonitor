# MCSrvMonitor

A high-performance, minimalistic Minecraft server status monitor and web dashboard.

## Features

- [x] Efficient asynchronous polling of Minecraft server status
- [x] Automatic storage to SQLite database
- [x] Lightweight web frontend for real-time status display
- [x] Simple configuration and easy deployment
- [ ] Both Java and Bedrock Edition server support
- [ ] Multiple server monitoring support
## Quick Start

### Requirements

- Rust 1.70+
- SQLite3

### Build & Run

```bash
git clone https://github.com/CaSilicate-dev/MCSrvMonitor.git
cd MCSrvMonitor/server
# Edit config.yaml as needed
cargo run --release
```

### Configuration

Example `config.yaml`:

```yaml
database_filename: "history.db"
port: 8000
addr: "0.0.0.0"
interval_sec: 5
server_addr: "server.fts427.top"
length: 1000
```

- `database_filename`: SQLite database file path
- `port`: API port
- `addr`: API address
- `interval_sec`: Sampling interval
- `server_addr`: Minecraft server address `(address:[port])`
- `length`: WebUI history length

### Project Structure

```text
.
├── frontend
│   ├── package.json
│   ├── package-lock.json
│   ├── public
│   │   ├── favicon.ico
│   │   ├── index.html
│   │   ├── logo192.png
│   │   ├── logo512.png
│   │   ├── manifest.json
│   │   └── robots.txt
│   ├── README.md
│   └── src
│       ├── App.css
│       ├── App.js
│       ├── App.test.js
│       ├── index.css
│       ├── index.js
│       ├── logo.svg
│       ├── reportWebVitals.js
│       └── setupTests.js
├── LICENSE
├── README.md
└── server
    ├── assets
    │   └── lang.json
    ├── Cargo.toml
    ├── config.yaml
    ├── src
    │   ├── backend.rs
    │   └── main.rs
    └── templates
        └── index.html.hbs
```

### License

MIT License © 2025 CaSilicate-dev
