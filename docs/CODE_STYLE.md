# 代码风格与命名规范

> 本文是**从当前代码归纳 + 后续统一约定**的规范。标注「已强制」的项目由 CI 校验；标注「约定」的项目当前靠 review 保证，尚未自动化。

## 1. 执行状态总览

| 项目 | 状态 | 校验方式 |
| --- | --- | --- |
| Rust 代码格式 | ✅ 已强制 | `cargo fmt --all -- --check`（`ci.yml` 的 `backend` job） |
| Rust 工具链版本 | ✅ 已固定 | CI 与文档统一 Rust **1.98.0**（rustfmt 版本差异会改变块注释排版） |
| 换行 / 缩进 / 编码 | ✅ 已强制 | `.editorconfig`（UTF-8、LF、4 空格；JS/Vue/YAML 2 空格）+ `.gitattributes` |
| Rust lint | ⚠️ 报告模式 | `cargo clippy --all-targets`，存量 15 条告警未阻断合并 |
| 前端代码格式 | ⚠️ 仅约定 | 无 ESLint / Prettier，靠 `.editorconfig` 与 review |
| 命名规范 | ⚠️ 仅约定 | 本文第 2 ~ 6 节，靠 PR review |
| 测试覆盖 | ⚠️ 0 个用例 | `cargo test --workspace` 通过但无实际断言 |

**演进顺序**：先清理 clippy 存量告警 → 启用 `-- -D warnings`；再引入 Prettier + ESLint，先只检查变更文件，最后全量格式化。

## 2. Rust 命名

| 对象 | 规则 | 现有示例 |
| --- | --- | --- |
| crate / 模块 | `snake_case`，单词语义单一 | `erp_backend`、`entities`、`user_management_handler` |
| 类型 / trait / 枚举 | `PascalCase` | `AppState`、`AppError`、`UserClaims` |
| 函数 / 变量 | `snake_case` | `create_router`、`connect_db`、`get_customer_stats` |
| 常量 / 静态量 | `UPPER_SNAKE_CASE` | `ROLE_ADMIN`、`TOKEN_EXPIRY_SECONDS`、`SERVICE_FACTOR_95` |
| Entity 文件 | `<表单数>_<...>.rs` | `customer.rs`、`purchase_order_item.rs` |
| Handler 文件 | `<领域>_handler.rs` | `inventory_handler.rs`、`workflow_handler.rs` |
| 测试模块 | `#[cfg(test)] mod tests`，函数 `test_<行为>` | — |

**约定**

- 业务常量集中放 `constants.rs`，禁止在 handler 里散落魔法数字与中文字面量。
- 新增模块必须在对应 `mod.rs` 显式 `pub mod` 导出；不要靠路径推断。
- 金额计算统一 `rust_decimal`，时间统一 `DateTimeUtc`（当前存量代码有 `f64` / `String`，新代码不得延续）。
- 错误统一走 `handlers/error.rs::AppError`，不要在 handler 里手工拼 `StatusCode`。
- 公开函数补文档注释说明「做什么 / 失败条件 / 副作用」，尤其是写库与发通知的 handler。
- 格式一律 `cargo fmt`，**不提交手工排版差异**；不使用 `#[rustfmt::skip]`，除非有明确理由并在 PR 中说明。

## 3. JavaScript / Vue 命名

| 对象 | 规则 | 现有示例 |
| --- | --- | --- |
| 组件文件 | `PascalCase.vue` | `HomeDashboard.vue`、`PurchaseManagement.vue` |
| 组件 `name` | 与文件同名 `PascalCase` | `name: 'HomeDashboard'` |
| 变量 / 函数 | `camelCase` | `pageSize`、`getUnreadCount` |
| 常量 | `UPPER_SNAKE_CASE` | `API_ENDPOINTS`、`PAGINATION`、`ORDER_STATUS` |
| 路由 path | 小写复数 / kebab-case | `/purchase`、`/inventory`、`/reports` |
| 工具模块 | `camelCase.js` | `request.js` |
| CSS 类 | kebab-case | — |

**约定**

- 所有网络请求必须经 `src/utils/request.js`，禁止在组件里直接 `axios.create` 或写完整 URL。
- 新增端点优先登记到 `src/api/constants.js`，并与后端 `routes.rs` 保持一致（当前存在偏差，见 [api.md](api.md#6-已知前后端契约偏差)）。
- 状态值等业务枚举以**后端 `constants.rs` 为准**，前端只做展示映射。
- 组件内避免超长单文件；超过约 500 行时拆 `components/` 子组件或 `composables/`。
- 缩进 2 空格、单引号、语句末不加分号（与现有 `request.js` / `constants.js` 风格一致）；**统一格式化工具引入前，只改你触碰的行**。

## 4. 数据库命名

| 对象 | 规则 | 示例 |
| --- | --- | --- |
| 表 | 复数 `snake_case` | `purchase_orders`、`inventory_adjustments` |
| 主键 | 统一 `id`（`i32`） | |
| 字段 | `snake_case`，可空用 `Option<T>` | `min_stock`、`related_order_type` |
| 外键字段 | `<被引用表单数>_id` | `customer_id`、`purchase_order_id` |
| 时间戳 | `created_at` / `updated_at`；完成类 `completed_at` / `approved_at` / `read_at` | |
| 布尔 | `is_<形容词>` | `is_read` |
| 唯一键 | 编码类字段声明 `#[sea_orm(unique)]` | `customer_code`、`supplier_code` |

新表必须同时：定义 Entity → `entities/mod.rs` 导出 → `database.rs::init_tables` 追加 → 更新 [database.md](database.md)。详见 [数据库设计](database.md#7-已知设计问题)。

## 5. API 命名

| 对象 | 规则 | 示例 |
| --- | --- | --- |
| 前缀 | 一律 `/api` | |
| 集合 | 复数 kebab-case | `/api/purchase-orders`、`/api/inventory/items` |
| 单项 | `/api/<集合>/{id}` | `/api/customers/{id}` |
| 动作 | `POST /api/<集合>/{id}/<动词>` | `/api/purchase-orders/{id}/approve` |
| 统计 | `/api/<集合>/stats` | `/api/sales-orders/stats` |
| 导出 | `/api/reports/export/<类型>` | `/api/reports/export/finance` |
| 查询参数 | `snake_case` | `page`、`page_size` |
| JSON 字段 | `snake_case` | `order_number`、`created_by` |

**约束**：新增路由必须在 `routes.rs` 明确归入公开 / 登录用户 / 管理员三组之一；管理操作不得放进 `protected_routes`。新增接口同步更新 [api.md](api.md)。

## 6. Git 与文件组织

### 6.1 分支与提交

- 主分支 `main`，受保护，禁止直推；功能分支 `feat/<slug>`、`fix/<slug>`、`docs/<slug>`、`refactor/<slug>`、`chore/<slug>`、`ci/<slug>`、`test/<slug>`。
- 提交信息遵循 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/v1.0.0/)：`<type>(<scope>): <中文简述>`，如 `fix(inventory): 修复安全库存计算除零`。
- 标签 `v<MAJOR>.<MINOR>.<PATCH>`，附注标签（`-a`），详见 [RELEASING.md](RELEASING.md)。

### 6.2 目录职责

| 路径 | 放什么 | 不放什么 |
| --- | --- | --- |
| `erp-backend/src/` | Rust 源码 | 文档、配置样例 |
| `erp-frontend/src/` | 前端源码 | 构建产物 `dist/` |
| `docs/` | 设计、部署、规范、治理、发版文档 | 临时报告、个人笔记 |
| `readmes/` | 按场景切分的上手卡片 | 长篇设计文档 |
| `deploy/` | compose 环境示例、运维脚本 | 含真实口令的文件 |
| `.github/` | workflows、issue/PR/discussion 模板、CODEOWNERS、dependabot | 业务代码 |
| 根目录 | `README.md`、`LICENSE`、`CITATION.cff`、manifest、忽略规则、Docker 编排文件 | 散落的治理文档 |

> GitHub 会在 `docs/` 下自动识别 `CONTRIBUTING.md`、`SECURITY.md`、`CODE_OF_CONDUCT.md`，因此治理文档集中到 `docs/` 不会丢失贡献 / 安全入口。

### 6.3 绝不入库

`.env`、`.env.*`（`.env.example` 除外）、`*.pem` / `*.key` / `*.p12`、`logs/` 与 `*.log`、`*.dump` / `*.sql.gz` / `*.db`、`node_modules/` / `dist/` / `target/`、`.workbuddy/`。规则见 `.gitignore`。

## 7. Review 检查清单

- [ ] `cargo fmt --all -- --check` 通过，且未引入新 clippy 告警
- [ ] 命名符合第 2 ~ 5 节；金额 / 时间类型没有新增 `f64` / `String`
- [ ] 新增路由权限分组正确；新增表已登记 `init_tables`
- [ ] 新增环境变量写入 `.env.example`，**仅占位值**
- [ ] 前端依赖变更同步 `package-lock.json`，后端依赖变更同步 `Cargo.lock`
- [ ] 相关文档（`docs/` 下对应文件与 [CHANGELOG.md](CHANGELOG.md)）已更新
- [ ] 不含任何真实口令、令牌、私钥、日志或业务数据
