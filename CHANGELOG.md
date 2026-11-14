# 更新日志

本文件遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

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
