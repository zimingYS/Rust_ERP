# 安全政策

> 本文件是 GitHub **Security** 标签页的入口。**完整安全政策、已知问题清单与密钥管理规范见 [docs/SECURITY.md](../docs/SECURITY.md)**；两者如有差异，以完整版为准。

## 报告漏洞

**请勿为安全问题开公开 Issue 或 Pull Request。**

优先使用私有通道：仓库 → **Security** → **Advisories** → **Report a vulnerability**（<https://github.com/zimingYS/Rust_ERP/security/advisories/new>）。若不可用，请通过 GitHub 私信维护者 `@zimingYS`。

报告请包含：影响版本、漏洞类型、复现步骤、潜在影响、建议修复（如有）。**不要附带真实口令、生产数据或可直接利用的攻击脚本**。首次响应尽量在 7 天内，修复通常随下一个补丁版本发布，披露在修复发布后进行。

## 支持范围

| 版本 | 是否提供安全修复 |
| --- | --- |
| `1.0.x` | ✅ |
| `< 1.0.0` | ❌ |

本项目为毕业设计作品，安全修复以尽力而为为原则，不提供服务等级承诺。

## 已知安全问题（v1.0.0 必读）

以下是**已公开、已记录**的现状，重复报告会被关闭；欢迎提交修复 PR。

| 编号 | 位置 | 问题 | 影响 |
| --- | --- | --- | --- |
| S-01 | `erp-backend/src/auth.rs:6` | `JWT_SECRET` 硬编码为 `"your-secret-key"` | 任何人可离线签发任意 `role_id` 的令牌，直接获得管理员权限 |
| S-02 | `erp-backend/src/handlers/user_management_handler.rs:289` | 默认 / 重置口令 `123456` | 账号可被猜到 |
| S-03 | `erp-backend/src/main.rs` | CORS `allow_origin(Any)` | 任意站点可发起跨站请求 |
| S-04 | 全局 | 无速率限制、无账号锁定、无令牌吊销 | 登录接口可被暴力破解 |
| S-05 | `logs/` 与 `audit_logs` | 审计与访问日志可能记录请求参数 | 日志外泄会暴露业务数据 |
| S-06 | 整体 | 仅监听 `127.0.0.1:3000`，无 TLS、无迁移工具 | 反向代理配置不当时易被直接暴露 |

> **结论：v1.0.0 不得部署到公网，也不得存放真实业务数据。**

修复建议、密钥管理规范见 [docs/SECURITY.md](../docs/SECURITY.md)；密钥与依赖审计结果、修复优先级见 [docs/SECURITY_AUDIT.md](../docs/SECURITY_AUDIT.md)。
