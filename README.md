# Rust_ERP

[![CI](https://github.com/zimingYS/Rust_ERP/actions/workflows/ci.yml/badge.svg)](https://github.com/zimingYS/Rust_ERP/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/zimingYS/Rust_ERP)](https://github.com/zimingYS/Rust_ERP/releases)
[![License](https://img.shields.io/badge/license-PolyForm%20Noncommercial%201.0.0-blue)](LICENSE)

基于 **Rust（Axum + SeaORM + PostgreSQL）** 与 **Vue 3（Vite + Element Plus + ECharts）** 实现的企业资源计划（ERP）系统，覆盖采购、销售、库存、财务、客户与供应商、工作流审批、消息通知、审计与报表等模块。

> 本项目为**毕业设计作品**，定位是学习、演示与二次开发参考。**请勿部署到公网或用于生产环境**，原因见[安全说明](#安全说明与已知限制)。

---

## 许可说明

本项目采用 [PolyForm Noncommercial License 1.0.0](LICENSE)（可自由分享、禁止商业使用）：

| 允许 | 禁止 |
| --- | --- |
| 阅读、学习、复制、修改源码 | 任何**商业目的**的使用 |
| 在非商业场景下分享、再发布 | 为获取商业优势或报酬而使用本软件 |
| 基于本项目创作非商业衍生作品 | 向第三方提供本软件用于商业目的 |

- 这是**源码可见（source-available）的非商业许可**，不是 OSI 认证的开源许可证，GitHub 会显示为 `Other`。
- 商业使用须另行取得作者书面授权。
- 提交 Pull Request 即表示你同意所贡献的内容按同一许可证授权，且你有权这样授权。

---

## 技术栈

| 层 | 选型 |
| --- | --- |
| 后端 | Rust edition 2024、Axum 0.8、Tokio、tower-http（CORS / trace） |
| 数据 | PostgreSQL、SeaORM 1.1（sqlx-postgres + rustls）、rust_decimal |
| 认证 | jsonwebtoken（JWT）、argon2 口令哈希、角色中间件 |
| 导入导出 | calamine（Excel）、csv |
| 前端 | Vue 3、Vite 7、Vue Router 4、Element Plus、ECharts、Axios |
| 工程化 | GitHub Actions（CI / Release 产物）、Dependabot、rustfmt、clippy |

## 功能概览

| 模块 | 主要能力 |
| --- | --- |
| 认证与权限 | 注册 / 登录、JWT、管理员 / 主管 / 普通用户三级角色 |
| 基础资料 | 客户、供应商、物品管理 |
| 采购与销售 | 采购订单、销售订单、订单明细、状态流转 |
| 库存 | 出入库、盘点调整、安全库存预警、补货计算 |
| 财务与报表 | 收支流水、财务报表、Excel 导入 |
| 工作流 | 流程实例、审批步骤、待办处理 |
| 运营支撑 | 仪表盘、消息中心、通知、审计日志与活动汇总 |

后端共暴露 17 组 `/api/*` 路由，前端接口常量集中在 `erp-frontend/src/api/constants.js`。

## 目录结构

```
Rust_ERP/
├── Cargo.toml                 # Workspace（成员：erp-backend）
├── erp-backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs            # 启动入口，监听 127.0.0.1:3000
│       ├── routes.rs          # 路由装配与权限分层
│       ├── database.rs        # 连接、建表、默认角色初始化
│       ├── entities/          # SeaORM 实体
│       ├── handlers/          # 各业务模块 handler
│       ├── middleware.rs      # JWT / 角色校验
│       └── audit_middleware.rs# 审计日志
├── erp-frontend/
│   ├── src/api/constants.js   # 接口路径常量
│   ├── src/utils/request.js   # Axios 实例（baseURL: http://127.0.0.1:3000）
│   └── src/views/             # 页面组件
├── .github/                   # CI、Issue/PR 模板、CODEOWNERS、Dependabot
└── docs/MAINTAINERS.md        # 维护者手册（公开、分支保护、发版）
```

## 快速开始

**前置条件**：Rust ≥ 1.85（edition 2024）、Node.js ≥ 20.19、npm、可本地连接的 PostgreSQL。

```bash
# 1. 建库
createdb erp_db            # 或在 psql 中执行 CREATE DATABASE erp_db;

# 2. 配置连接串（.env 已被忽略，切勿提交）
cp .env.example .env
#   编辑 .env 中的 DATABASE_URL

# 3. 启动后端（首次启动自动建表并写入 3 个默认角色）
cargo run -p erp-backend

# 4. 另开终端启动前端
cd erp-frontend
npm ci
npm run dev
```

- 后端：`http://127.0.0.1:3000`，健康检查 `GET /api/health`
- 前端：Vite 默认 `http://localhost:5173`；`src/utils/request.js` 已直连后端 3000 端口，无需额外代理
- 首次启动**不预置账号**：先用 `/api/register` 注册，再在用户管理中把角色调整为管理员
- 后端以 `tracing-appender` 按天写日志到 `logs/`，该目录已被忽略

## 参与贡献

欢迎提交 Issue、功能建议与 Pull Request。主分支受保护，**不接受直接推送**，流程如下：

1. Fork 仓库并基于 `master` 建分支（`feat/xxx`、`fix/xxx`、`docs/xxx`）
2. 本地跑通自检（见 [CONTRIBUTING.md](CONTRIBUTING.md)）
3. 提交 PR，说明改动与验证方式，关联对应 Issue
4. 等待 `backend` / `frontend` 两项 CI 全绿 + 至少 1 位 Code Owner 审批
5. 由维护者合并

贡献规范、提交信息约定、行为准则分别见 [CONTRIBUTING.md](CONTRIBUTING.md) 与 [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)。安全漏洞**不要**开公开 Issue，请走 [SECURITY.md](SECURITY.md) 中的私有通道。

## 安全说明与已知限制

v1.0.0 是学习与演示版本，存在以下**明确已知的安全缺口**：

- `erp-backend/src/auth.rs` 将 `JWT_SECRET` 硬编码为源码常量，任何人都可伪造管理员令牌
- `erp-backend/src/handlers/user_management_handler.rs` 中默认口令为 `123456`
- CORS 放行任意来源，且无速率限制、无刷新令牌与吊销机制
- 无自动化测试用例，无数据库迁移工具，表结构由启动时 `init_tables` 创建
- 服务仅监听 `127.0.0.1:3000`

因此**不要将其暴露到公网或存放真实业务数据**。完整说明与修复建议见 [SECURITY.md](SECURITY.md)。

## 版本与发版

当前版本 **v1.0.0**（2026-11-14），变更记录见 [CHANGELOG.md](CHANGELOG.md)。发布流程与分支保护配置见 [docs/MAINTAINERS.md](docs/MAINTAINERS.md)。

## 引用

如在课程作业或研究中参考本项目，请使用仓库根目录的 `CITATION.cff`（GitHub 侧栏提供 "Cite this repository" 入口）。

## 许可证

[PolyForm Noncommercial License 1.0.0](LICENSE) © 2026 zimingYS。可自由分享与修改，**禁止商业使用**。
