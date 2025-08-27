[English](../README.md) | 简体中文

# MC 服务器在线率监视器

一个高精度、高性能、极简主义的 Minecraft 服务器状态监控器与 Web 仪表盘。

## 功能特性

- [x] 高效的异步 Minecraft 服务器状态轮询
- [x] 自动存储至 SQLite 数据库
- [x] 轻量级 Web 前端，实时显示状态
- [x] 简单的配置与便捷的部署
- [ ] 支持 Java 与基岩版服务器
- [ ] 支持多服务器监控

## 快速开始

### 依赖要求

- Rust 1.70+
- SQLite3

### 构建与运行

```bash
git clone https://github.com/CaSilicate-dev/MCSrvMonitor.git
cd MCSrvMonitor/server
# 根据需要编辑 config.yaml
cargo run --release
```

# 配置

示例 `config.yaml`:

```yaml
database_filename: "history.db"
port: 8000
addr: "0.0.0.0"
interval_sec: 5
server_addr: "server.fts427.top"
length: 1000
```

- `database_filename`: SQLite 数据库文件路径
- `port`: API 端口
- `addr`: API 监听地址
- `interval_sec`: 采样间隔（秒）
- `server_addr`: Minecraft 服务器地址 `(address:[port])`
- `length`: WebUI 历史记录长度

### 项目结构

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

### 开源协议

MIT License © 2025 CaSilicate-dev
