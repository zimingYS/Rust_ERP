# Docker 速查卡片

完整说明见 [docs/deployment.md](../docs/deployment.md)，镜像发布见 [docs/RELEASING.md](../docs/RELEASING.md)。

## 一条命令起整套

```bash
cp deploy/compose.env.example deploy/compose.env
docker compose --env-file deploy/compose.env up -d --build
```

| 服务 | 访问 |
| --- | --- |
| 前端 nginx | `http://localhost:8080` |
| 后端（经 nginx 反代） | `http://localhost:8080/api/health` |
| PostgreSQL | 仅容器内 `postgres:5432` |

## ⚠️ 起不来的第一原因

v1.0.0 源码有两处硬编码，标准容器网络下必须**自行修改**（本轮未改业务源码）：

| 位置 | 要改成什么 |
| --- | --- |
| `erp-backend/src/main.rs:48` | 监听地址由 `127.0.0.1` 改为 `0.0.0.0`（建议读取环境变量） |
| `erp-frontend/src/utils/request.js:6` | `baseURL` 由 `http://127.0.0.1:3000` 改为相对路径 `/api` 或 `import.meta.env.VITE_API_BASE_URL` |

详见 [部署文档 §5.2](../docs/deployment.md#52-部署前置修改必须自行完成)。

## 常用操作

```bash
docker compose ps                    # 状态
docker compose logs -f backend       # 后端日志
docker compose exec postgres pg_isready -U postgres
docker compose up -d --build backend # 只重建后端
docker compose down                  # 停止，保留数据卷
docker compose down -v               # ⚠️ 删除数据库数据卷
```

## 文件对照

| 文件 | 作用 |
| --- | --- |
| `Dockerfile` | 后端多阶段构建（Rust 1.98 → debian-slim） |
| `erp-frontend/Dockerfile` | 前端多阶段构建（Node 22 → nginx） |
| `erp-frontend/nginx.conf` | SPA 回退 + `/api` 反代 |
| `docker-compose.yml` | postgres / backend / frontend 编排 |
| `.dockerignore` | 排除 target、node_modules、logs、.env、.git |
| `deploy/compose.env.example` | 变量占位示例 |

> 不要把本项目部署到公网：`JWT_SECRET` 硬编码、默认口令、CORS 全放行，见 [docs/SECURITY.md](../docs/SECURITY.md)。
