# 更新日志

本文件遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 新增

- **文档体系**：新建 `docs/` 设计文档（系统架构、功能模块设计、数据库设计、API 接口）、部署文档、代码风格与命名规范、安全检查报告、版本与发版规范；新建 `readmes/` 分场景上手卡片与文档索引
- **Docker 部署**：后端 / 前端多阶段 Dockerfile、nginx 反代配置、`docker-compose.yml`（postgres + backend + frontend）、`.dockerignore`、compose 变量示例
- **CI 增强**：`security.yml`（密钥扫描、Cargo / npm 依赖审计、PR 依赖差异审查）、`docker-publish.yml`（GHCR 镜像构建与可选 SSH 部署）
- **社区模板**：Discussion 分类模板、文档类 Issue 模板、CODEOWNERS 与 PR 自检项更新
- **目录重组**：治理文档迁入 `docs/`，根目录只保留 README / LICENSE / CITATION.cff

### 修复

- **依赖安全**：`npm audit fix`（不加 `--force`）将前端漏洞 **10 项 → 0**，`package.json` 未变，仅更新 `package-lock.json`；`npm run build` 通过
- **依赖安全**：`cargo update` + `calamine 0.34.0 → 0.38.0`（连带 `zip 9.5.1`、`quick-xml 0.42.0`）将后端 RUSTSEC **20 条 → 3 条**；`cargo check --workspace --all-targets` 通过，业务源码零改动
- **PR 清理**：统一关闭首轮 19 个陈旧 Dependabot PR 并删除全部来源分支，远端仅保留 `main`
- 试验性升级 `jsonwebtoken 11.1.0` 因无安全收益（仍带 `rsa 0.9.10`）已回滚，保留 10.4.0

### 变更

- 默认分支由 `master` 统一为 `main`，全部文档与工作流引用同步
- `dependabot.yml` 增加 docker 生态，并收敛为**每月一次、每生态最多 3 个 PR、minor/patch 分组、屏蔽 `semver-major`**
- `security.yml` 的 `npm-audit` 由报告模式升级为 **high+ 阻断合并**；`rust-audit` 因剩余 3 条存量保持报告模式
- `.gitignore` 增加 `.workbuddy/` 与 `deploy/compose.env`
- `docs/SECURITY_AUDIT.md` 按修复后复跑数据重写

### 已知限制

- 容器化前仍需自行修改两处硬编码（`main.rs` 监听地址、`request.js` 的 `baseURL`），见 `docs/deployment.md` §5.2
- 依赖审计存量：npm **0** 项、Cargo **3** 条 RUSTSEC（`rsa` / `crypto-bigint` / `libsqlite3-sys`，均需上游 major 才能修且不在当前业务主路径），见 `docs/SECURITY_AUDIT.md`

---

## [1.0.0] - 2026-11-14

首个公开发行版本。

### 新增

- **后端**：Axum 0.8 + SeaORM 1.1 REST 服务，覆盖 17 组 `/api/*` 路由
- **认证授权**：注册 / 登录、JWT、管理员 / 主管 / 普通用户三级角色中间件
- **业务模块**：客户、供应商、物品、库存预警与补货、采购订单、销售订单、财务收支、报表、仪表盘
- **流程与运营**：工作流引擎与审批步骤、消息中心与通知、审计日志中间件与活动汇总
- **数据导入**：Excel / CSV 导入（calamine + csv）
- **前端**：Vue 3 + Vite 7 + Element Plus + ECharts 单页应用
- **工程化**：GitHub Actions CI 与 Release 产物工作流、Issue / PR 模板、CODEOWNERS、Dependabot、贡献与安全规范、许可证、统一换行与忽略规则

### 变更

- 清洗了 Git 历史中误提交的 `.env` 数据库连接串，改为 `.env.example` 模板
- 纳入全量 rustfmt 格式化结果，CI 增加 `cargo fmt --check` 门禁
- 移除残留的 Vite 模板 README，补齐项目文档

### 已知限制

- `JWT_SECRET` 为源码硬编码常量，可伪造令牌 → **禁止公网 / 生产部署**
- 用户管理默认口令 `123456`；CORS 放行任意来源；无速率限制
- 自动化测试数量为 0；无数据库迁移工具，表结构在启动时创建
- 服务仅监听 `127.0.0.1:3000`

完整清单见 [SECURITY.md](SECURITY.md)。

[1.0.0]: https://github.com/zimingYS/Rust_ERP/releases/tag/v1.0.0
