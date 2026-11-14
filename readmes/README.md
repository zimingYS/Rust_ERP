# 分场景 README

这里放**按场景切分的上手卡片**，与 `docs/` 的设计文档互补：`docs/` 讲「为什么这样设计」，`readmes/` 讲「我现在要做某件事，该看哪几个文件」。

| 场景 | 卡片 | 适合谁 |
| --- | --- | --- |
| 只改后端接口 / 实体 | [backend.md](backend.md) | 后端开发、接口联调 |
| 只改前端页面 / 请求 | [frontend.md](frontend.md) | 前端开发、UI 调整 |
| 用容器把整套跑起来 | [docker.md](docker.md) | 部署、演示环境准备 |

## 三句话了解项目

1. 后端是单个 Axum 二进制 `erp-backend`，启动时连 PostgreSQL、自动建表、监听 `127.0.0.1:3000`。
2. 前端是 Vite 构建的 Vue 3 SPA，通过 Axios 直连后端 3000 端口，JWT 存 `localStorage.erp_token`。
3. 两者都在同一个 Cargo workspace / Git 仓库里，没有独立的包发布流程。

## 常用命令速查

| 目的 | 命令 |
| --- | --- |
| 后端开发启动 | `cargo run -p erp-backend` |
| 后端全量自检 | `cargo fmt --all -- --check && cargo check --workspace --all-targets && cargo test --workspace` |
| 前端开发启动 | `cd erp-frontend && npm ci && npm run dev` |
| 前端生产构建 | `cd erp-frontend && npm run build` |
| 整套容器启动 | `docker compose up -d --build` |
| 依赖安全体检 | `cargo audit` 与 `cd erp-frontend && npm audit` |

> 完整说明见 [docs/deployment.md](../docs/deployment.md)，规范见 [docs/CODE_STYLE.md](../docs/CODE_STYLE.md)。
