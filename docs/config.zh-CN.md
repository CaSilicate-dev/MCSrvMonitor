[English](docs/config.md) | 简体中文

```markdown
# MCSrvMonitor 配置说明 (简体中文)  

本文件说明 **MCSrvMonitor** 的 JSON 配置。

---

## 顶层字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `port` | number | 监控服务（后端 API）监听端口，这里是 `18650` |
| `length` | number | Web 前端显示的历史数据长度，这里是 8640 条 |

---

## 后端配置

| 字段 | 类型 | 说明 |
|------|------|------|
| `dbfile` | string | SQLite 数据库文件路径，例如 `history.db` |
| `interval` | number | 轮询 Minecraft 服务器状态的时间间隔（秒），这里是每秒一次 |

> 后端负责采集服务器状态并存入数据库。

---

## 前端配置

| 字段 | 类型 | 说明 |
|------|------|------|
| `addr` | string | Web 前端绑定的 IP 地址，例如 `0.0.0.0` 表示监听所有网络接口 |
| `port` | number | Web 前端访问端口，例如 `21700` |

> 前端提供 Web 界面显示实时和历史状态。

---

## 服务器列表

每个元素表示一台要监控的 Minecraft 服务器。

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | string | 自定义服务器名称，用于前端显示或日志，例如 `fts` |
| `addr` | string | 服务器地址（域名或 IP），可带端口，例如 `server.fts427.top` |

> 可以添加多台服务器进行监控。

---

## 示例配置

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
