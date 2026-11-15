# Rust_ERP

<p align="center">
  <strong>基于 Rust 与 Vue 3 的企业资源计划管理系统</strong>
  <br />
  面向学习、课程设计与二次开发的全栈 ERP 项目
</p>

<p align="center">
  <a href="https://github.com/zimingYS/Rust_ERP/actions/workflows/ci.yml">
    <img src="https://github.com/zimingYS/Rust_ERP/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI" />
  </a>
  <a href="https://github.com/zimingYS/Rust_ERP/actions/workflows/security.yml">
    <img src="https://github.com/zimingYS/Rust_ERP/actions/workflows/security.yml/badge.svg?branch=main" alt="Security" />
  </a>
  <a href="https://github.com/zimingYS/Rust_ERP/releases">
    <img src="https://img.shields.io/github/v/release/zimingYS/Rust_ERP" alt="Release" />
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-PolyForm%20Noncommercial%201.0.0-blue" alt="License" />
  </a>
</p>

---

## 项目简介

**Rust_ERP** 是一个基于现代 Rust 后端与 Vue 3 前端构建的企业资源计划（ERP）管理系统。

后端采用 **Rust + Axum + SeaORM + PostgreSQL**，前端采用 **Vue 3 + Vite + Element Plus + ECharts**，围绕企业日常运营场景，整合采购、销售、库存、财务、客户与供应商、工作流审批、消息通知、审计及报表等业务模块。

项目采用前后端分离架构，注重模块划分、代码规范、工程化实践与文档完整性，可作为 Rust Web 开发、全栈项目实践及 ERP 业务系统设计的参考。

> [!IMPORTANT]
> **项目定位：毕业设计作品，仅供学习、演示与二次开发参考。**
>
> 当前版本存在已知安全缺口，**请勿部署到公网、用于生产环境或存放真实业务数据**。具体风险与限制请阅读[安全说明与已知限制](#安全说明与已知限制)。

## 项目概览

| 项目 | 说明 |
|:---|:---|
| 项目名称 | Rust_ERP |
| 当前版本 | `v1.0.0` |
| 发布日期 | 2026-11-14 |
| 项目类型 | 前后端分离 · 企业资源计划管理系统 |
| 后端 | Rust · Axum · SeaORM |
| 前端 | Vue 3 · Vite · Element Plus |
| 数据库 | PostgreSQL |
| 开源协议 | PolyForm Noncommercial License 1.0.0 |

## 功能模块

系统围绕企业业务流程组织功能，各模块相互协作，形成基础业务管理闭环。

| 模块 | 核心功能 |
|:---|:---|
| **认证与权限** | 用户注册、登录、JWT 身份认证、管理员 / 主管 / 普通用户三级角色 |
| **基础资料** | 客户管理、供应商管理、物品信息管理 |
| **采购与销售** | 采购订单、销售订单、订单明细、订单状态流转 |
| **库存管理** | 商品出入库、库存盘点、安全库存预警、补货计算 |
| **财务与报表** | 收支流水、财务报表、Excel 数据导入 |
| **工作流审批** | 流程实例、审批步骤、待办事项处理 |
| **运营支撑** | 数据仪表盘、消息中心、通知管理、审计日志、活动汇总 |

## 技术栈

项目采用前后端分离架构，后端负责业务逻辑、数据持久化与权限控制，前端负责交互展示与数据可视化。

| 层次 | 技术选型 |
|:---|:---|
| **后端语言** | Rust · Edition 2024 |
| **Web 框架** | Axum 0.8 · Tokio · tower-http |
| **数据访问** | PostgreSQL · SeaORM 1.1 · sqlx-postgres |
| **数据类型** | rust_decimal |
| **身份认证** | JWT · jsonwebtoken · Argon2 |
| **数据导入导出** | calamine · csv |
| **前端框架** | Vue 3 · Vite 7 |
| **UI 组件** | Element Plus |
| **数据可视化** | ECharts |
| **前端基础设施** | Vue Router 4 · Axios |
| **工程化与运维** | GitHub Actions · Dependabot · Docker · Docker Compose |
| **代码质量** | rustfmt · Clippy |

## 快速开始

### 1. 环境要求

开始之前，请确保本地已安装以下工具：

| 工具 | 版本要求 |
|:---|:---|
| Rust | `1.98.0` |
| Node.js | `≥ 20.19` |
| npm | 与 Node.js 配套 |
| PostgreSQL | 已安装并运行 |

### 2. 获取项目

```bash
git clone https://github.com/zimingYS/Rust_ERP.git
cd Rust_ERP
```

### 3. 配置数据库

创建 PostgreSQL 数据库：

```bash
createdb erp_db
```

复制环境变量模板：

```bash
cp .env.example .env
```

根据本地数据库配置修改 `.env` 中的 `DATABASE_URL`。

### 4. 启动后端

在项目根目录执行：

```bash
cargo run -p erp-backend
```

后端默认地址：

```text
http://127.0.0.1:3000
```

健康检查接口：

```http
GET /api/health
```

首次启动时，程序会自动创建数据库表并写入 3 个默认角色。**系统不会预置用户账号**，需要通过 `/api/register` 注册用户后再登录。

### 5. 启动前端

打开新的终端窗口：

```bash
cd erp-frontend
npm ci
npm run dev
```

前端开发服务器默认地址：

```text
http://localhost:5173
```

> [!NOTE]
> 完整安装步骤、环境变量说明、Docker 部署流程及相关注意事项，请参阅[部署文档](docs/deployment.md)。

## 项目结构

项目采用 Cargo Workspace 管理 Rust 后端，并将前端、文档及部署配置独立组织。

```text
Rust_ERP/
│
├── .github/                     # GitHub Actions、模板、CODEOWNERS、Dependabot
│
├── erp-backend/                 # Rust 后端
│   └── src/
│       ├── main.rs              # 程序入口
│       ├── routes/              # 路由配置
│       ├── database/            # 数据库连接与初始化
│       ├── entities/            # 数据实体
│       ├── handlers/            # 业务处理器
│       └── middleware/          # 中间件
│
├── erp-frontend/                # Vue 3 前端
│   ├── src/
│   │   ├── views/               # 页面视图
│   │   ├── router/              # 路由配置
│   │   ├── api/                 # 接口请求
│   │   └── utils/               # 通用工具
│   └── Dockerfile               # 前端构建与 Nginx 配置
│
├── docs/                        # 项目设计、开发与治理文档
│
├── readmes/                     # 分场景 README
│
├── deploy/                      # Docker Compose 示例与运维脚本
│
├── Cargo.toml                   # Cargo Workspace
├── Dockerfile                   # 后端多阶段构建
├── docker-compose.yml           # PostgreSQL + Backend + Frontend
└── LICENSE                      # 项目许可证
```

> 顶层目录职责、代码组织方式及命名约定见[代码风格与命名规范](docs/CODE_STYLE.md)。

## 项目文档

所有项目文档统一维护在 [`docs/`](docs/README.md) 目录下，按照用途划分为四个类别。

| 分类 | 文档 | 主要内容 |
|:---|:---|:---|
| **系统设计** | [系统架构](docs/architecture.md) · [功能模块设计](docs/modules.md) · [数据库设计](docs/database.md) · [API 接口](docs/api.md) | 分层架构、模块职责、15 张数据表、`/api/*` 接口 |
| **使用与部署** | [部署指南](docs/deployment.md) · [分场景 README](readmes/README.md) | 安装、构建、启动、Docker 部署、模块速查 |
| **工程规范** | [代码规范](docs/CODE_STYLE.md) · [贡献指南](docs/CONTRIBUTING.md) · [行为准则](docs/CODE_OF_CONDUCT.md) | 代码格式、命名约定、分支管理、提交规范、PR 流程 |
| **治理与运维** | [安全政策](docs/SECURITY.md) · [安全审计报告](docs/SECURITY_AUDIT.md) · [发版规范](docs/RELEASING.md) · [维护者手册](docs/MAINTAINERS.md) · [更新日志](docs/CHANGELOG.md) | 安全风险、依赖审计、发版流程、分支保护与回滚 |

## 工程化与协作

项目使用 GitHub Actions 建立自动化工作流，并通过 Dependabot 跟踪依赖更新。

| 工程能力 | 说明 |
|:---|:---|
| **CI** | 自动执行后端与前端检查 |
| **Security** | 依赖与安全相关检查 |
| **Release** | 版本发布工作流 |
| **Container** | Docker 镜像构建 |
| **Dependency Updates** | Dependabot 依赖更新 |
| **Code Quality** | rustfmt 与 Clippy |

### 贡献流程

主分支 `main` 受分支保护规则约束，**不接受直接推送**。欢迎通过 Fork 和 Pull Request 参与项目。

1. **Fork**：Fork 本仓库。
2. **Branch**：从 `main` 创建功能分支，例如 `feat/xxx`、`fix/xxx`、`docs/xxx`。
3. **Develop**：在本地完成代码修改，并执行必要的自检。
4. **Pull Request**：提交 PR，说明改动内容、实现目的与验证方式，并关联对应 Issue。
5. **Review**：等待 `backend`、`frontend` 两项 CI 检查通过，并至少获得 1 位 Code Owner 审批。
6. **Merge**：由维护者完成合并。

贡献前请阅读[贡献指南](docs/CONTRIBUTING.md)与[行为准则](docs/CODE_OF_CONDUCT.md)。

### 交流与问题反馈

- **功能建议与方案讨论**：[GitHub Discussions](https://github.com/zimingYS/Rust_ERP/discussions)
- **Bug 反馈**：通过 GitHub Issues 提交。
- **安全漏洞**：请勿通过公开 Issue 披露，按照[安全政策](docs/SECURITY.md)中的私有通道报告。

## 安全说明与已知限制

> [!CAUTION]
> **当前版本 `v1.0.0` 不适用于生产环境。**
>
> 项目仍存在明确的安全缺口。在相关问题修复并完成安全验证之前，请勿将服务暴露到公网，也不要存放真实业务数据。

目前已知的问题如下：

| 风险项 | 当前情况 |
|:---|:---|
| **JWT 密钥** | `erp-backend/src/auth.rs:6` 将 `JWT_SECRET` 硬编码为源码常量，攻击者获取源码后可伪造管理员令牌 |
| **默认口令** | `erp-backend/src/handlers/user_management_handler.rs:289` 存在默认口令 `123456` |
| **CORS 配置** | 允许任意来源跨域请求 |
| **请求防护** | 尚未实现速率限制 |
| **令牌管理** | 尚未实现刷新令牌与令牌吊销机制 |
| **自动化测试** | 尚无自动化测试用例 |
| **数据库迁移** | 尚未引入数据库迁移工具，表结构由启动时的 `init_tables` 创建 |
| **网络监听** | 服务仅监听 `127.0.0.1:3000` |
| **前端 API 地址** | `baseURL` 写死为 `127.0.0.1:3000`，容器化部署前需要手动修改 |

### 依赖安全审计

当前存量依赖审计结果：

- npm：**0 项**安全告警。
- Cargo：**3 条 RUSTSEC 告警**，均涉及需要上游 major 版本升级才能修复的传递依赖，暂不在当前业务主路径中。

详细审计结果、风险分析及修复建议见[安全审计报告](docs/SECURITY_AUDIT.md)与[安全政策](docs/SECURITY.md)。

## 版本与更新

<details>
<summary><strong>当前版本：v1.0.0</strong> · 2026-11-14</summary>

本版本为项目首个正式标记版本，具体改动与版本记录见：

- [CHANGELOG.md](docs/CHANGELOG.md)
- [RELEASING.md](docs/RELEASING.md)

</details>

## 引用与许可证

### 引用项目

如在课程作业、学术研究或其他技术文档中参考本项目，请使用仓库根目录下的 [`CITATION.cff`](CITATION.cff)。

GitHub 页面侧栏提供 **Cite this repository** 入口，可用于生成引用信息。

### 许可证

本项目采用 [PolyForm Noncommercial License 1.0.0](LICENSE)。

**允许：**
- 阅读与学习项目源码。
- 在许可范围内修改项目。
- 分享与传播项目及其修改版本。

**禁止：**
- 将项目用于任何商业目的。

该许可证属于 **Source-Available（源码可见）许可证**，并非 OSI 认可的开源许可证。因此，GitHub 可能将其显示为 `Other`。

向本项目提交 Pull Request，即表示你同意所贡献的内容按照同一许可证进行授权。

---

<p align="center">
  <sub>Built with Rust · Vue 3 · PostgreSQL</sub>
  <br />
  <sub>学习、实践与探索现代全栈开发</sub>
</p>
