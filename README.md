English | [简体中文](docs/README.zh-CN.md)

# MCSrvMonitor

A high-accuracy, high-performance, minimalistic Minecraft server status monitor and web dashboard.

## Features

- [x] Efficient asynchronous polling of Minecraft server status
- [x] Automatic storage to SQLite database
- [x] Lightweight web frontend for real-time status display
- [x] Simple configuration and easy deployment
- [ ] Both Java and Bedrock Edition server support
- [x] Multiple server monitoring support
## Quick Start

### Requirements

- Rust 1.70+
- SQLite3
- Nodejs
- Make

### Build & Run

```bash
git clone https://github.com/CaSilicate-dev/MCSrvMonitor.git
cd MCSrvMonitor
# Edit config.yaml as needed
make start
```

#### Manual build

```bash
git clone https://github.com/CaSilicate-dev/MCSrvMonitor.git
cd MCSrvMonitor/server
cd frontend && npm install && npm run build
cd ../server
# Edit config.yaml as needed
cargo run --release
```



### Configuration

Example `config.yaml`:

```json
{
    "port": 18650,
    "length": 8640,
    "backend": {
        "dbfile": "history.db",
        "interval": 1
    },
    "frontend": {
        "addr": "0.0.0.0",
        "port": 21700
    },
    "servers": [
        {
            "name": "fts",
            "addr": "server.fts427.top"
        },
        {
            "name": "local",
            "addr": "server.fts427.top"
        }
    ]
}
```

[Configuration Explained docs/config.md](docs/config.md)

## Project Structure

```text
.
├── assets
│   └── lang.json
├── Cargo.toml
├── config.yaml
├── docs
│   └── README.zh-CN.md
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
├── Makefile
├── README.md
└── src
    ├── backend.rs
    ├── frontend.rs
    └── main.rs

```

## License

MIT License © 2025 CaSilicate-dev
