# 后端模块速查

面向「我要改后端某个功能」的场景。设计层面的说明见 [docs/architecture.md](../docs/architecture.md)，表结构见 [docs/database.md](../docs/database.md)。

## 入口与分层

```
erp-backend/src/
├── main.rs                 # 入口：日志 → 建库/建表 → AppState → CORS/审计中间件 → 路由 → 监听
├── routes.rs               # 唯一路由装配点，按「公开 / 管理员 / 登录用户」三层分组
├── state.rs                # AppState { db: DatabaseConnection }，以 Arc 共享
├── database.rs             # connect_db / init_tables / init_default_roles / initialize_database
├── constants.rs            # 角色 ID、订单状态、业务参数、JWT 有效期（86400s）
├── auth.rs                 # JWT 签发与 UserClaims 定义（JWT_SECRET 为源码常量）
├── middleware.rs           # auth_middleware（校验 Bearer）、require_admin_middleware（role_id==1）
├── audit_middleware.rs     # 全局审计：记录 method/path/状态码/UA/IP 到 audit_logs
├── permissions.rs          # 权限辅助
├── entities/               # 15 个 SeaORM Entity，一张表一个文件
└── handlers/               # 17 个业务 handler 模块 + error.rs + health.rs
```

## 改一个功能时该动哪些文件

| 改动 | 涉及文件 |
| --- | --- |
| 新增 API | `handlers/<module>.rs` 加函数 → `handlers/mod.rs` 注册 → `routes.rs` 挂路由 → [docs/api.md](../docs/api.md) 补文档 |
| 新增表 / 字段 | `entities/<name>.rs` 加 Entity → `entities/mod.rs` 导出 → `database.rs::init_tables` 加建表 → [docs/database.md](../docs/database.md) 补文档 |
| 改权限 | `middleware.rs` + `routes.rs` 的分组归属（管理员路由必须挂 `require_admin_middleware`） |
| 改业务常量 | `constants.rs`（订单状态、库存参数、通知优先级等集中在这里） |
| 改启动行为 | `main.rs`（监听地址、CORS、日志） |

## 关键事实

- **权限分层在 `routes.rs`**：`admin_routes` 与 `protected_routes` 各自 `.layer(auth_middleware)`，管理员组再叠 `require_admin_middleware`；`/api/health`、`/api/register`、`/api/login` 完全公开。
- **建表不用迁移文件**：`database.rs::init_tables` 用 `Schema::create_table_from_entity(...).if_not_exists()` 逐张建表，共 15 张，顺序有依赖含义（role → user → items → 订单 …）。已有表不会被修改，**加字段不会自动生效**，需手工 ALTER。
- **默认角色**：启动时写入 `roles` 三条记录，ID 固定 `1=管理员 / 2=主管 / 3=普通用户`；**不预置账号**。
- **金额类型不统一**：`customers/suppliers.credit_limit` 用 `Decimal`，而订单金额与流水用 `f64` —— 新代码涉及金额优先用 `rust_decimal`，见 [docs/CODE_STYLE.md](../docs/CODE_STYLE.md)。
- **时间类型不统一**：部分表用 `DateTimeUtc`，部分用 `String`；新增字段优先 `DateTimeUtc`。
- **日志**：`tracing_appender::rolling::daily("logs", "backend.log")`，`logs/` 已被忽略，**不得提交**。

## 自检

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets --message-format=short
cargo audit
```

> CI 固定 Rust **1.98.0**：不同版本 rustfmt 对块注释排版不同，换版本会让 `cargo fmt --check` 失败。
> 当前测试数为 0、clippy 有存量告警，详见 [docs/SECURITY_AUDIT.md](../docs/SECURITY_AUDIT.md)。
