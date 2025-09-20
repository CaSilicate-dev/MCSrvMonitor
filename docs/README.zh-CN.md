[English](../README.md) | 简体中文

# MC 服务器在线率监视器后端程序

一款高精度、高性能、轻量型的我的世界（Minecraft）服务器状态监控工具。

## 功能特性

- [x] 高效的异步 Minecraft 服务器状态轮询
- [x] 自动存储至 SQLite 数据库
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
# 根据需要编辑 config.json
cargo run --release
```

# 配置

示例 `config.json`:

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
[配置教程 docs/config.zh-CN.md](./config.zh-CN.md)
