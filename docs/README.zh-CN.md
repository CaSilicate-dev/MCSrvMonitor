[English](../README.md) | 简体中文

# MC 服务器在线率监视器

一个高精度、高性能、极简主义的 Minecraft 服务器状态监控器与 Web 仪表盘。

## 功能特性

- [x] 高效的异步 Minecraft 服务器状态轮询
- [x] 自动存储至 SQLite 数据库
- [x] 轻量级 Web 前端，实时显示状态
- [x] 简单的配置与便捷的部署
- [ ] 支持 Java 与基岩版服务器
- [x] 支持多服务器监控

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

示例 `config.json`:

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
[配置教程 docs/config.zh-CN.md](./config.zh-CN.md)

### 项目结构

```text
.
├── assets
│   └── lang.json
├── Cargo.lock
├── Cargo.toml
├── config.json
├── config.yaml
├── docs
│   ├── config.md
│   ├── config.zh-CN.md
│   └── README.zh-CN.md
├── frontend
│   ├── build
│   ├── node_modules
│   ├── package.json
│   ├── package-lock.json
│   ├── public
│   ├── README.md
│   └── src
├── history.db
├── LICENSE
├── Makefile
├── README.md
├── src
│   ├── backend.rs
│   ├── frontend.rs
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    ├── debug
    └── release
```

### 开源协议

MIT License © 2025 CaSilicate-dev
