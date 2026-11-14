# 文档索引

本目录是 `Rust_ERP` 的**唯一文档中心**。根目录 `README.md` 只保留项目概览与快速入口。

## 按角色选文档

| 你是… | 先读 |
| --- | --- |
| 第一次接触项目、想跑起来 | [安装 / 构建 / 启动](deployment.md) → [分场景 README](../readmes/README.md) |
| 要改后端或前端某个模块 | [功能模块设计](modules.md) → [代码风格与命名规范](CODE_STYLE.md) |
| 要改接口或联调 | [API 接口文档](api.md) → [数据库设计](database.md) |
| 想理解整体设计 | [系统架构](architecture.md) |
| 想贡献代码 | [贡献指南](CONTRIBUTING.md) → [行为准则](CODE_OF_CONDUCT.md) |
| 关心安全或发现了漏洞 | [安全政策](SECURITY.md) → [密钥与依赖检查报告](SECURITY_AUDIT.md) |
| 维护者 / 要发版 | [维护者手册](MAINTAINERS.md) → [版本与发版](RELEASING.md) |
| 想了解版本变化 | [更新日志](CHANGELOG.md) |

## 文档清单

### 系统设计

| 文件 | 内容 | 主要来源 |
| --- | --- | --- |
| [architecture.md](architecture.md) | 分层架构、请求生命周期、中间件链、状态与依赖、部署视图 | `erp-backend/src/main.rs`、`routes.rs` |
| [modules.md](modules.md) | 后端 17 组 handler 与前端 13 个视图的模块职责、交互关系 | `erp-backend/src/handlers/`、`erp-frontend/src/router/index.js` |
| [database.md](database.md) | 15 张表的字段、类型、主外键关系、状态枚举、建表顺序 | `erp-backend/src/entities/`、`database.rs` |
| [api.md](api.md) | 全部 `/api/*` 端点、HTTP 方法、权限层级、认证方式与已知契约偏差 | `erp-backend/src/routes.rs` |

### 使用与部署

| 文件 | 内容 |
| --- | --- |
| [deployment.md](deployment.md) | 本地安装 / 构建 / 启动、环境变量、Docker Compose、部署前置修改、故障排查 |

### 工程规范

| 文件 | 内容 |
| --- | --- |
| [CODE_STYLE.md](CODE_STYLE.md) | Rust / JavaScript / 数据库 / API / Git 的统一格式与命名规范，以及当前执行状态 |
| [CONTRIBUTING.md](CONTRIBUTING.md) | 开发环境、分支与提交规范、自检清单、PR 流程 |
| [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) | 贡献者行为准则 |

### 治理与运维

| 文件 | 内容 |
| --- | --- |
| [SECURITY.md](SECURITY.md) | 漏洞报告通道、已知安全问题 S-01 ~ S-06、密钥管理规范 |
| [SECURITY_AUDIT.md](SECURITY_AUDIT.md) | 全历史密钥扫描结论、npm / Cargo 依赖审计结果、CI 门禁策略 |
| [RELEASING.md](RELEASING.md) | 语义化版本、版本号同步点、打标签与发布流程、产物与镜像 |
| [MAINTAINERS.md](MAINTAINERS.md) | 仓库设置、分支保护规则、Discussions、回滚与后续发版 |
| [CHANGELOG.md](CHANGELOG.md) | 按版本记录的新增 / 变更 / 已知限制 |

## 约定

- 文档一律使用**简体中文**，代码块与标识符保持英文原样。
- 文档中对源码的引用必须精确到**文件路径 + 行号**（行号随版本漂移，仅作定位参考）。
- 现状与设计不一致时，文档以**当前代码为准**并显式标注「已知偏差」，不美化、不静默修正。
- 新增文档必须在本索引中登记；删除文档需同步清理所有引用链接。
- GitHub 会在 `docs/` 下自动识别 `CONTRIBUTING.md`、`SECURITY.md`、`CODE_OF_CONDUCT.md`，因此这些文件从根目录迁入后，贡献与安全入口仍然有效。
