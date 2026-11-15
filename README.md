# Rust_ERP

[![CI](https://github.com/zimingYS/Rust_ERP/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/zimingYS/Rust_ERP/actions/workflows/ci.yml)
[![Security](https://github.com/zimingYS/Rust_ERP/actions/workflows/security.yml/badge.svg?branch=main)](https://github.com/zimingYS/Rust_ERP/actions/workflows/security.yml)
[![Release](https://img.shields.io/github/v/release/zimingYS/Rust_ERP)](https://github.com/zimingYS/Rust_ERP/releases)
[![License](https://img.shields.io/badge/license-PolyForm%20Noncommercial%201.0.0-blue)](LICENSE)

基于 **Rust（Axum 0.8 + SeaORM 1.1 + PostgreSQL）** 与 **Vue 3（Vite 7 + Element Plus + ECharts）** 实现的企业资源计划（ERP）系统，覆盖采购、销售、库存、财务、客户与供应商、工作流审批、消息通知、审计与报表等模块。

> 本项目为**毕业设计作品**，定位是学习、演示与二次开发参考。**请勿部署到公网或用于生产环境**，原因见 [安全说明](#安全说明与已知限制)。

---

## 快速开始

**前置条件**：Rust **1.98.0**、Node.js ≥ 20.19、npm、PostgreSQL。

```bash
createdb erp_db
cp .env.example .env            # 填写本地 DATABASE_URL
cargo run -p erp-backend        # 后端 http://127.0.0.1:3000
cd erp-frontend && npm ci && npm run dev   # 前端 http://localhost:5173
```

健康检查：`GET http://127.0.0.1:3000/api/health`。首次启动自动建表并写入 3 个默认角色，**不预置账号**，先走 `/api/register`。

完整步骤、Docker 部署、环境变量说明见 **[docs/deployment.md](docs/deployment.md)**。

## 文档地图

全部文档集中在 [`docs/`](docs/README.md)，按用途分为四类：

| 类别 | 文档 | 内容 |
| --- | --- | --- |
| 系统设计 | [系统架构](docs/architecture.md) · [功能模块设计](docs/modules.md) · [数据库设计](docs/database.md) · [API 接口](docs/api.md) | 分层架构、模块职责、15 张表、全部 `/api/*` 端点 |
| 使用与部署 | [安装 / 构建 / 启动 / Docker](docs/deployment.md) · [分场景 README](readmes/README.md) | 本地开发、容器编排、前后端模块速查 |
| 工程规范 | [代码风格与命名规范](docs/CODE_STYLE.md) · [贡献指南](docs/CONTRIBUTING.md) · [行为准则](docs/CODE_OF_CONDUCT.md) | rustfmt、命名约定、分支与提交规范、PR 流程 |
| 治理与运维 | [安全政策](docs/SECURITY.md) · [密钥与依赖检查报告](docs/SECURITY_AUDIT.md) · [版本与发版](docs/RELEASING.md) · [维护者手册](docs/MAINTAINERS.md) · [更新日志](docs/CHANGELOG.md) | 已知风险、审计结果、发版流程、分支保护与回滚 |

## 技术栈

| 层 | 选型 |
| --- | --- |
| 后端 | Rust edition 2024、Axum 0.8、Tokio、tower-http（CORS / trace） |
| 数据 | PostgreSQL、SeaORM 1.1（sqlx-postgres + rustls）、rust_decimal |
| 认证 | jsonwebtoken（JWT）、argon2 口令哈希、角色中间件 |
| 导入导出 | calamine（Excel）、csv |
| 前端 | Vue 3、Vite 7、Vue Router 4、Element Plus、ECharts、Axios |
| 工程化 | GitHub Actions（CI / 安全 / Release / 镜像）、Dependabot、rustfmt、clippy、Docker Compose |

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

## 目录结构

```
Rust_ERP/
├── Cargo.toml                  # Workspace（成员：erp-backend）
├── erp-backend/src/            # main / routes / database / entities / handlers / middleware
├── erp-frontend/src/           # views / router / api / utils
├── docs/                       # 设计文档、部署、安全、发版与治理文档
├── readmes/                    # 分场景上手说明
├── deploy/                     # compose 环境示例与运维脚本
├── Dockerfile                  # 后端多阶段构建
├── erp-frontend/Dockerfile     # 前端构建 + nginx
├── docker-compose.yml          # postgres + backend + frontend 编排
└── .github/                    # workflows / issue & PR & discussion 模板 / CODEOWNERS / dependabot
```

顶层目录职责与命名约定见 [docs/CODE_STYLE.md](docs/CODE_STYLE.md)。

## 参与贡献

主分支 `main` **受保护，不接受直接推送**。公开仓库中任何人都可以 Fork 并开 PR：

1. Fork 并基于 `main` 建分支（`feat/xxx`、`fix/xxx`、`docs/xxx`）
2. 本地跑通自检（见 [贡献指南](docs/CONTRIBUTING.md)）
3. 提交 PR，说明改动与验证方式，关联对应 Issue
4. 等待 `backend` / `frontend` 两项 CI 全绿、所有 review 对话解决（单人维护者仓库不要求人工审批）
5. 由维护者合并

使用咨询与方案讨论请发到 [Discussions](https://github.com/zimingYS/Rust_ERP/discussions)。安全漏洞**不要**开公开 Issue，请走 [安全政策](docs/SECURITY.md) 中的私有通道。

## 安全说明与已知限制

v1.0.0 是学习与演示版本，存在以下**明确已知的安全缺口**：

- `erp-backend/src/auth.rs:6` 将 `JWT_SECRET` 硬编码为源码常量，任何人都可伪造管理员令牌
- `erp-backend/src/handlers/user_management_handler.rs:289` 中默认口令为 `123456`
- CORS 放行任意来源，且无速率限制、无刷新令牌与吊销机制
- 无自动化测试用例，无数据库迁移工具，表结构由启动时 `init_tables` 创建
- 服务仅监听 `127.0.0.1:3000`；前端 `baseURL` 亦写死该地址（容器化前置修改见 [部署文档](docs/deployment.md#52-部署前置修改必须自行完成)）
- 依赖审计存量：npm **0** 项、Cargo **3** 条 RUSTSEC 告警（均为需上游 major 才能修的传递依赖，不在当前业务主路径），详见 [检查报告](docs/SECURITY_AUDIT.md)

因此**不要将其暴露到公网或存放真实业务数据**。完整说明与修复建议见 [docs/SECURITY.md](docs/SECURITY.md)。

## 版本与发版

当前版本 **v1.0.0**（2026-11-14），变更记录见 [更新日志](docs/CHANGELOG.md)，发版规范见 [docs/RELEASING.md](docs/RELEASING.md)。

## 引用与许可

如在课程作业或研究中参考本项目，请使用仓库根目录的 `CITATION.cff`（GitHub 侧栏提供 "Cite this repository" 入口）。

本项目采用 [PolyForm Noncommercial License 1.0.0](LICENSE)：**可自由阅读、学习、修改与分享，禁止任何商业目的使用**。这是源码可见（source-available）的非 OSI 许可证，GitHub 会显示为 `Other`；提交 Pull Request 即表示你同意所贡献内容按同一许可证授权。
