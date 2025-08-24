# MCSrvMonitor

A high-performance, minimalistic Minecraft server status monitor and web dashboard.

## Features

- Efficient asynchronous polling of Minecraft server status
- Automatic storage to SQLite database
- Lightweight web frontend for real-time status display
- Simple configuration and easy deployment
- Both Java and Bedrock Edition server support

## Quick Start

### Requirements

- Rust 1.70+
- SQLite3

### Build & Run

```bash
git clone https://github.com/CaSilicate-dev/MCSrvMonitor.git
cd MCSrvMonitor
cp config.yaml.example config.yaml
# Edit config.yaml as needed
cargo run --release
```

### Configuration

Example `config.yaml`:

```yaml
db: "./test.db"
interval: 60
mc:
  java: true
  addr: "127.0.0.1"
  port: 25565
web:
  addr: "127.0.0.1"
  port: 80
```

- `db`: SQLite database file path
- `interval`: Polling interval in seconds
- `mc.java`: If Java Edition server
- `mc.addr`/`mc.port`: Minecraft server address and port
- `web.addr`/`web.port`: Web server bind address and port

### Project Structure

```text
.
├── src/
│   ├── collector.rs    # Polling logic
│   ├── config.rs       # Configuration parsing
│   ├── lib.rs          # Library
│   ├── main.rs         # Main function
│   └── web.rs          # Web server
├── assets/
│   ├── lang.yaml       # Web data content
│   └── style.css       # Main style sheets
├── templates/
│   └── index.html.hbs  # Recket index page template
├── .gitignore
├── Cargo.toml
├── config.yaml.example # Configuration template
├── README.md
├── LICENSE
```

### License

MIT License © 2025 CaSilicate-dev
