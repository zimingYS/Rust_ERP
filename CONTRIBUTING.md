# 贡献指南

感谢你愿意为 Rust_ERP 提交代码、文档或想法。本项目采用 [PolyForm Noncommercial License 1.0.0](LICENSE)，参与贡献前请先阅读 [行为准则](CODE_OF_CONDUCT.md) 与 [README 的许可说明](README.md#许可说明)。

## 1. 贡献前必读

- **提交 PR 即表示**：你有权以本项目许可证授权你提交的内容，且同意该内容按同一许可证（可分享、禁止商业使用）对外提供。
- **不要提交**：你无权公开的代码、第三方专有代码、真实业务数据、`.env`、口令、令牌、私钥、日志、数据库转储。
- **安全漏洞**不要开公开 Issue，请按 [SECURITY.md](SECURITY.md) 走私有报告通道。
- 使用咨询、方案讨论请发到 Discussions，不要占用 Issue。

## 2. 开发环境

前置：Rust ≥ 1.85、Node.js ≥ 20.19、npm、PostgreSQL。

```bash
cp .env.example .env          # 填写本地 DATABASE_URL
cargo run -p erp-backend      # 127.0.0.1:3000
cd erp-frontend && npm ci && npm run dev
```

详细启动说明见 [README](README.md#快速开始)。

## 3. 分支与提交规范

- 主分支 `master` **受保护，禁止直接推送**；请 Fork 后从 `master` 建分支。
- 分支命名：`feat/xxx`、`fix/xxx`、`docs/xxx`、`refactor/xxx`、`chore/xxx`。
- 提交信息遵循 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/v1.0.0/)：

```
feat(inventory): 增加安全库存预警阈值配置
fix(auth): 修复令牌过期后未返回 401
docs: 补充本地启动说明
```

常用类型：`feat` / `fix` / `docs` / `refactor` / `test` / `chore` / `ci` / `build` / `perf`。

## 4. 提交前自检

在项目根目录依次执行，全部通过再推分支：

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cd erp-frontend && npm ci && npm run build
```

补充约定：

- Rust 代码以 `rustfmt` 默认风格为准，不要提交手工排版差异。
- `cargo clippy --workspace --all-targets` 目前仍有存量告警，CI 以**报告模式**运行、不阻断合并；**请尽量不要新增告警**。
- 前端依赖变更必须同步提交 `erp-frontend/package-lock.json`。
- 新增环境变量要同步更新 `.env.example`（只写占位值）。
- 行为变更请更新 `README.md` / `CHANGELOG.md`。

## 5. Pull Request 流程

1. 在 Fork 中完成开发并跑通自检。
2. 开 PR，**base 必须是 `zimingYS/Rust_ERP:master`**，填写 PR 模板并关联 Issue。
3. 等待 CI：`backend`、`frontend` 两项状态检查必须全绿。
4. 主分支保护规则要求：至少 1 位 Code Owner 批准、所有 review 对话已解决、分支与主分支保持同步；管理员同样受约束。
5. 由维护者用 squash / merge 方式合并，合并后自动删除来源分支。
6. 若你的 PR 涉及 CI、发版或许可配置，请在描述中额外说明理由。

## 6. Issue 规范

- 缺陷：使用「🐛 缺陷报告」模板，提供复现步骤、期望 / 实际行为、环境信息与脱敏日志。
- 功能建议：使用「✨ 功能建议」模板，说清问题背景、期望方案、影响模块。
- 提交前先搜索是否已有重复 Issue。

## 7. 代码审查关注点

- 是否引入新的安全缺口（尤其是密钥硬编码、越权、SQL 拼接、日志泄露）。
- 财务与库存数值是否使用 `rust_decimal`，避免浮点误差。
- 权限分层是否正确：管理员路由必须挂 `require_admin_middleware`。
- 是否保持「原始状态可持久化、派生值运行时计算」的一致风格。

维护者应在合理时间内给出首次响应；若 PR 长期无回应，可在 Issue 中礼貌提醒。
