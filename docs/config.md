English | [简体中文](docs/config.zh-CN.md)

# MCSrvMonitor Configuration Explained (English)  

This document explains the JSON configuration for **MCSrvMonitor** in English.

---

## Top-Level Fields

| Field | Type | Description |
|-------|------|-------------|
| `port` | number | API port for the monitoring service (here `18650`) |
| `length` | number | Number of historical data points shown in the frontend (here `8640`) |

---

## Backend Section

| Field | Type | Description |
|-------|------|-------------|
| `dbfile` | string | SQLite database file path for storing historical status (e.g., `history.db`) |
| `interval` | number | Polling interval in seconds for querying Minecraft servers (here `1`) |

> The backend collects server status and stores it in the database.

---

## Frontend Section

| Field | Type | Description |
|-------|------|-------------|
| `addr` | string | IP address the web frontend binds to (e.g., `0.0.0.0` listens on all interfaces) |
| `port` | number | Port for accessing the frontend (e.g., `21700`) |

> The frontend provides a web interface for real-time and historical status.

---

## Servers Array

Each element represents a Minecraft server to monitor.  

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Custom server name for display and logs (e.g., `fts`) |
| `addr` | string | Server address (domain or IP), optionally with port (e.g., `server.fts427.top`) |

> Multiple servers can be added for monitoring.

---

## Example Configuration

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
