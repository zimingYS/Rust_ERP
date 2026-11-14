# 安装、构建、启动与 Docker 部署

## 1. 环境要求

| 组件 | 版本 | 说明 |
| --- | --- | --- |
| Rust | **1.98.0** | edition 2024；CI 固定此版本以保证 rustfmt 结果一致 |
| Node.js | ≥ 20.19（CI 用 22） | npm 随附 |
| PostgreSQL | 14+ | 需可本地连接，数据库名默认 `erp_db` |
| Docker / Compose | 可选 | 容器方案，本机验证过 Docker 29 / Compose v5 |

固定 Rust 版本：

```bash
rustup toolchain install 1.98.0 --profile minimal -c rustfmt -c clippy
rustup override set 1.98.0
```

## 2. 本地开发启动

```bash
# 1) 建库
createdb erp_db                   # 或 psql: CREATE DATABASE erp_db;

# 2) 配置连接串
cp .env.example .env              # .env 已被忽略，严禁提交
#    编辑 DATABASE_URL

# 3) 启动后端（首次启动自动建 15 张表 + 3 条默认角色）
cargo run -p erp-backend

# 4) 另开终端启动前端
cd erp-frontend
npm ci
npm run dev
```

| 服务 | 地址 | 验证 |
| --- | --- | --- |
| 后端 | `http://127.0.0.1:3000` | `curl http://127.0.0.1:3000/api/health` → `ERP 系统运行正常！` |
| 前端 | `http://localhost:5173` | 打开 `/login` |

**首次使用**：系统不预置账号，先 `POST /api/register` 注册，再由管理员在「用户管理」中把 `role_id` 调整为 `1`。

日志写到 `logs/backend.log.<日期>`（按天滚动），目录已忽略。

## 3. 环境变量

| 变量 | 必填 | 读取位置 | 说明 |
| --- | --- | --- | --- |
| `DATABASE_URL` | ✅ | `erp-backend/src/database.rs::connect_db` | `postgres://<user>:<password>@<host>:5432/erp_db`；缺失直接 panic |
| `RUST_LOG` | ❌ | `tracing_subscriber::EnvFilter` | 日志级别，如 `info`、`erp_backend=debug` |
| `JWT_SECRET` | ❌（当前**不读取**） | — | v1.0.0 的密钥是 `auth.rs:6` 源码常量；改为环境变量注入后需在此登记 |
| `VITE_API_BASE_URL` | ❌（当前**不读取**） | — | 建议引入，用于替换 `utils/request.js` 中写死的后端地址 |

`.env.example` 只保留占位值；新增配置项必须同步该文件。`.env` 与任何真实凭据都不得进入 Git，见 [SECURITY_AUDIT.md](SECURITY_AUDIT.md)。

## 4. 生产构建

```bash
# 后端二进制
cargo build --release -p erp-backend
# 产物：target/release/erp-backend

# 前端静态资源
cd erp-frontend && npm ci && npm run build
# 产物：erp-frontend/dist/
```

`dist/` 是纯静态 SPA，可由任意 Web 服务器托管；使用 `vue-router` 的 history 模式时，**所有非 API 路径需回退到 `index.html`**（下面的 nginx 配置已处理）。

## 5. Docker 部署方案

### 5.1 组成

| 文件 | 作用 |
| --- | --- |
| `Dockerfile` | 后端多阶段构建：`rust:1.98-bookworm` 编译 → `debian:bookworm-slim` 运行 |
| `erp-frontend/Dockerfile` | 前端多阶段构建：`node:22-alpine` 构建 → `nginx:alpine` 托管 |
| `erp-frontend/nginx.conf` | SPA 回退 + `/api` 反向代理到 `backend:3000` |
| `docker-compose.yml` | `postgres` + `backend` + `frontend` 三服务编排 |
| `.dockerignore` | 排除 `target/`、`node_modules/`、`logs/`、`.env`、`.git/` |
| `deploy/compose.env.example` | compose 变量占位示例（可复制为 `deploy/compose.env`） |

```mermaid
flowchart LR
    U["浏览器 :8080"] --> N["frontend<br/>nginx"]
    N -->|"/api/* 反向代理"| B["backend<br/>Axum :3000"]
    B -->|"DATABASE_URL"| P[("postgres :5432")]
    P --- PV[("volume: erp_pgdata")]
```

| 服务 | 镜像 / 构建 | 端口 | 说明 |
| --- | --- | --- | --- |
| `postgres` | `postgres:16-alpine` | 容器内 5432 | 数据卷持久化，健康检查 `pg_isready` |
| `backend` | 根目录 `Dockerfile` | 容器内 3000 | `depends_on: postgres (healthy)` |
| `frontend` | `erp-frontend/Dockerfile` | 宿主 `8080 → 80` | `depends_on: backend` |

### 5.2 部署前置修改（必须自行完成）

> ⚠️ **v1.0.0 源码有两处硬编码，导致标准 bridge 网络下容器无法被外部访问。本轮按约定未修改任何业务源码，请在部署前自行完成这两处修改。**

| # | 位置 | 现状 | 问题 | 目标 |
| --- | --- | --- | --- | --- |
| 1 | `erp-backend/src/main.rs:48` | `SocketAddr::from(([127, 0, 0, 1], 3000))` | 绑定容器回环地址，其他容器与宿主机都连不上 | 改为监听 `[0, 0, 0, 0]`；更推荐从环境变量读取监听地址 |
| 2 | `erp-frontend/src/utils/request.js:6` | `baseURL: 'http://127.0.0.1:3000'` | 浏览器会请求**访问者自己的** 127.0.0.1，远程访问必失败 | 改为相对路径 `/api`（配合 nginx 反代），或读取 `import.meta.env.VITE_API_BASE_URL` |

改完后 `cargo fmt --all -- --check` 与 `npm run build` 复验。若暂不想改源码，只能让前后端共享网络命名空间（`network_mode: host`，仅 Linux 主机有效），**不推荐**用于演示之外的场景。

### 5.3 启动

```bash
cp deploy/compose.env.example deploy/compose.env   # 按需填写，已被忽略
docker compose --env-file deploy/compose.env up -d --build
docker compose ps
docker compose logs -f backend
```

访问 `http://localhost:8080`；接口经 nginx 转发到 `http://localhost:8080/api/*`。

停止 / 清理：

```bash
docker compose down              # 保留数据卷
docker compose down -v           # ⚠️ 会删除数据库数据卷，谨慎
```

### 5.4 验证

```bash
curl -i http://localhost:8080/api/health
curl -i -X POST http://localhost:8080/api/login \
  -H 'Content-Type: application/json' -d '{"username":"...","password":"..."}'
docker compose exec postgres pg_isready -U postgres
```

## 6. 故障排查

| 现象 | 原因 | 处理 |
| --- | --- | --- |
| `DATABASE_URL not set` panic | 未配置 `.env` / compose 变量 | 复制 `.env.example` 并填写 |
| 后端日志 `PoolTimedOut` | PostgreSQL 未就绪或地址错误（容器内不能用 `localhost`） | compose 中用服务名 `postgres`；确认 healthcheck 通过 |
| 前端能打开但接口 502 | 未做 §5.2 第 1 项修改，backend 只监听回环 | 改监听 `0.0.0.0` |
| 页面调接口打到访问者本机 | 未做 §5.2 第 2 项修改 | 改用相对路径 `/api` 或 `VITE_API_BASE_URL` |
| 刷新子路由 404 | nginx 未配置 SPA 回退 | 使用仓库内的 `nginx.conf` |
| 加字段后后端取不到新列 | `init_tables` 用 `if_not_exists`，不修改已有表 | 手工 `ALTER TABLE`，见 [database.md](database.md#7-已知设计问题) |
| `cargo fmt --check` 在 CI 失败 | 本地 rustfmt 版本不是 1.98.0 | `rustup override set 1.98.0` |
| 前端构建内存不足 | Vite / ECharts 依赖较大 | 提高 Node 内存或改用 CI 预构建产物 |

## 7. 安全边界

- 本项目 **不得部署到公网或承载真实业务数据**：`JWT_SECRET` 硬编码、默认口令、CORS 全放行、无 TLS 与限流，详见 [SECURITY.md](SECURITY.md)。
- compose 中的数据库口令仅用于本地演示，请勿复用真实口令；`deploy/compose.env` 已被忽略。
- 容器镜像仅推送到 GHCR 作为版本产物，**不自动部署到公网**，自动部署 job 默认关闭，见 [RELEASING.md](RELEASING.md#4-自动部署可选默认关闭)。
