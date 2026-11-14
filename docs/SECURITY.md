# 安全政策

## 支持范围

| 版本 | 是否提供安全修复 |
| --- | --- |
| `1.0.x` | ✅ |
| `< 1.0.0` | ❌ |

本项目为毕业设计作品，安全修复以尽力而为为原则，不提供服务等级承诺。

## 报告漏洞

**请勿为安全问题开公开 Issue 或 Pull Request。**

请优先使用 GitHub 的私有通道：仓库 → **Security** → **Advisories** → **Report a vulnerability**（<https://github.com/zimingYS/Rust_ERP/security/advisories/new>）。若不可用，请通过 GitHub 私信维护者 `@zimingYS`。

报告请包含：影响版本、漏洞类型、复现步骤、潜在影响、建议修复（如有）。**不要附带真实口令、生产数据或可直接利用的攻击脚本**。

- 首次响应：尽量在 7 天内
- 确认与修复计划：视严重程度协商，修复通常随下一个补丁版本发布
- 披露：修复发布后再公开，除非双方另有约定

## 已知安全问题（v1.0.0 必读）

以下问题是**已公开、已记录**的现状，重复报告会被关闭；欢迎提交修复 PR。

| 编号 | 位置 | 问题 | 影响 | 建议 |
| --- | --- | --- | --- | --- |
| S-01 | `erp-backend/src/auth.rs:6` | `JWT_SECRET` 硬编码为 `"your-secret-key"` | 任何人可离线签发任意 `role_id` 的令牌，直接获得管理员权限 | 改为 `std::env::var("JWT_SECRET")` 注入，启动时校验长度与存在性，并轮换所有已签发令牌 |
| S-02 | `erp-backend/src/handlers/user_management_handler.rs:289` | 默认口令 `123456` | 重置 / 新建账号可被猜到 | 改为随机一次性口令 + 强制首次登录修改 |
| S-03 | `erp-backend/src/main.rs` CORS 配置 | `allow_origin(Any)` / `allow_methods(Any)` / `allow_headers(Any)` | 任意站点可携带凭据发起跨站请求 | 按部署域名配置白名单 |
| S-04 | 全局 | 无速率限制、无账号锁定、无令牌吊销 | 登录接口可被暴力破解，令牌泄露后无法失效 | 引入限流中间件与令牌黑名单 |
| S-05 | `logs/` | 审计与访问日志可能记录请求参数 | 日志外泄会暴露业务数据 | 脱敏后再落盘，并限制日志文件权限 |
| S-06 | 整体 | 仅监听 `127.0.0.1:3000`，无 TLS、无迁移工具 | 反向代理配置不当时易被直接暴露 | 仅在本机 / 内网运行 |

> **结论：v1.0.0 不得部署到公网，也不得存放真实业务数据。**

## 密钥管理规范

- `.env`、`.env.*`（`.env.example` 除外）、`*.pem`、`*.key` 均已被 `.gitignore` 忽略，**严禁提交**。
- 新增配置项一律在 `.env.example` 中提供**占位值**示例，不写真实值。
- 提交前自检：`git diff --cached` 中不得出现 `DATABASE_URL=` 的真实口令、令牌或私钥。
- 一旦误提交凭据：**先轮换**（改口令 / 吊销令牌），再清理历史，顺序不能反。轮换示例：

```sql
ALTER USER postgres WITH PASSWORD '新的强口令';
```

- 本项目在 v1.0.0 发布前已从全部 Git 历史中移除了误提交的 `.env`，但**原口令仍应视为已泄露并完成轮换**。

## 依赖安全

Dependabot 按生态检查 Cargo、npm、GitHub Actions 与 Docker 依赖，minor / patch 分组提交；带有 `dependencies` 标签的 PR 会经过常规 CI 与保护规则审查。

CI 中的 `security.yml` 每周与每次 PR 执行密钥扫描、`cargo audit`、`npm audit` 与依赖差异审查。存量告警与修复优先级见 [SECURITY_AUDIT.md](SECURITY_AUDIT.md)。
