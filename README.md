# hello

一个简单的 Rust Web 服务，使用 Axum 框架实现。

## 功能

- `GET /status` - 返回状态信息
- `GET /health` - 健康检查端点

## 运行

```bash
cargo run
```

服务将在 `0.0.0.0:3456` 启动。

## 依赖

- axum 0.8
- tokio
