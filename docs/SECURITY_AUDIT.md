# 密钥与依赖安全检查报告

**首次检查**：2026-11-14 · **复核**：2026-11-14（依赖修复后） · **基线**：`v1.0.0` / `main`

**当前结论**：Git 历史中**无真实凭据残留**；依赖告警已由 **npm 10 项 → 0**、**Cargo 20 条 → 3 条**。源码中仍有 **2 处硬编码凭据 / 弱口令**（属业务代码，本轮未修改），公开仓库后属于已披露信息。

> **2026-11-14 二次修订说明**：首轮报告记录的是「发现态」。当日晚间完成一次统一依赖修复：前端 `npm audit fix`（不加 `--force`）、后端 `cargo update` + 将 `calamine` 提升到 0.38.0，并同步清理首轮 19 个陈旧 Dependabot PR。以下数据均为**修复后复跑**结果。

## 1. 检查方法

| 范围 | 方法 |
| --- | --- |
| Git 全历史 | 对 `git rev-list --all` 的每个提交执行 `git grep`，模式覆盖 `postgres://user:pass@`、私钥头、`password = "..."`、`ghp_`、`sk-`；并逐提交用 `git ls-tree -r --name-only` 检查 `.env` 是否存在 |
| 工作区 | `git ls-files` 按敏感路径模式过滤；扫描未跟踪的 `.env*`、`logs/`、`*.log`、`*.2026-*`、`*.bak`、数据库转储 |
| 依赖 | `npm audit --json`（`erp-frontend`）、`cargo audit --no-fetch --json`（workspace）实跑 |
| 配置 | 复核 `.gitignore`、`.env.example`、CI 中的密钥处理 |

复现命令见 [§6](#6-复现命令)。

## 2. 密钥与敏感信息结果

| 编号 | 检查项 | 结果 | 判定 |
| --- | --- | --- | --- |
| K-01 | 历史中的 `.env` | 32+ 个提交逐棵扫描 `leak=0`；`git log origin/main -- .env` 为空 | ✅ 已清除（v1.0.0 发布前用 `filter-branch` 重写历史） |
| K-02 | 历史中的真实数据库连接串 | 全历史 `git grep` 无命中 | ✅ 无残留 |
| K-03 | `.env.example` | `DATABASE_URL=postgres://postgres:your_password@localhost:5432/erp_db`，`your_password` 为占位符 | ✅ 安全 |
| K-04 | 已跟踪的日志 / 转储 / 密钥文件 | `git ls-files` 中仅 `.env.example` 命中敏感模式，无 `logs/`、`*.log`、`*.dump`、`*.pem` | ✅ 无 |
| K-05 | 未跟踪的运行日志 | `logs/` 5 个文件 + `erp-backend/logs/` 6 个文件，内容为数据库错误与用户注册失败信息，未含口令；已被 `.gitignore` 忽略 | ✅ 不入库，但含业务错误信息，勿公开 |
| K-06 | `.env` 工作区文件 | 本地存在真实 `DATABASE_URL`，已被忽略；**历史中曾存在，旧口令应视为已泄露** | ⚠️ 需轮换 |
| K-07 | `JWT_SECRET` 硬编码 | `erp-backend/src/auth.rs:6` = `"your-secret-key"`，全历史可见 | ❌ S-01，**等同公开的签名密钥** |
| K-08 | 默认 / 重置口令 | `user_management_handler.rs:289` = `"123456"` | ❌ S-02 |
| K-09 | 审计日志字段 | `audit_logs.request_data` 可能记录请求参数 | ⚠️ S-05，建议写入前脱敏 |
| K-10 | 本地 AI 工作区 | `.workbuddy/`（含个人笔记） | ✅ 已加入 `.gitignore` |

**结论**：不存在需要再次重写 Git 历史的泄露。但 **K-07 与 K-08 是源码内的公开常量**，公开仓库后任何人都能离线签发 `role_id=1` 的管理员令牌。这是当前最高优先级风险，修复方式见 [SECURITY.md](SECURITY.md) S-01 / S-02。

> 按教练模式约定**未修改任何业务源码**，K-07 / K-08 的修复由仓库所有者自行完成。本轮仅调整依赖声明与锁文件。

## 3. 依赖安全：npm（已清零）

首轮 `npm audit`：**10 项**（high 8 / moderate 2）。修复后复跑：**0 项**（critical 0 / high 0 / moderate 0 / low 0）。

修复方式：`npm audit fix`（**不使用 `--force`**），全部在 `package.json` 已有的 `^` 约束内完成，`package.json` 内容未变，只更新 `package-lock.json`；`npm run build` 通过。

| 依赖 | 修复前 | 修复后 | 首轮主要问题 |
| --- | --- | --- | --- |
| `axios` | 1.15.0 | **1.21.1** | `validateStatus` 原型污染导致认证绕过；NO_PROXY 绕过 |
| `vue` / `@vue/server-renderer` | 3.5.27 | **3.5.45** | SSR 属性名黑名单缺失 CR 导致 XSS |
| `vite` | 7.3.2 | **7.3.8** | Windows 下 UNC 路径泄露 NTLMv2 哈希、参数转义不完整 |
| `echarts` | 6.0.0 | **6.1.0** | XSS |
| `postcss` | 8.5.6 | **8.5.33** | `</style>` 未转义 XSS；sourceMappingURL 任意文件读 |
| `source-map-js` | 1.2.1 | **1.2.2** | 构造 source map 造成事件循环 DoS |
| `form-data` | 4.0.5 | **4.0.6** | multipart 字段名 / 文件名 CRLF 注入 |
| `nanoid` | 3.3.11 | **3.3.20** | 非安全生成器在特定参数下无限循环 |
| `follow-redirects` | 1.15.11 | **1.16.3** | 跨域重定向泄露自定义认证头 |

> `vite` 停在 7.x，未升到 Dependabot 建议的 8.x（破坏性 major）；7.3.8 已覆盖全部告警。

## 4. 依赖安全：Cargo（20 → 3）

首轮 `cargo audit`：**20 条 RUSTSEC**。修复后复跑：**3 条**。

修复动作：

1. `cargo update`（仅在既有版本约束内更新 `Cargo.lock`）→ 清除 rustls / rustls-webpki / webpki-roots / rkyv 等 9 条；
2. 将 `erp-backend/Cargo.toml` 中 `calamine = "0.34.0"` 改为 `"0.38.0"` → 连带把 `zip 7.2.0 → 9.5.1`、`quick-xml 0.39.2 → 0.42.0` 拉上来，清除 7 条；`cargo check --workspace --all-targets` **通过，无需改动任何业务源码**（`import_handler.rs` 的 `open_workbook_auto` / `worksheet_range_at` API 兼容）；
3. 曾试升 `jsonwebtoken 10.4.0 → 11.1.0` 以清除 rsa 告警，实测 **11.1.0 仍依赖 `rsa 0.9.10` 与 `p256/p384 → crypto-bigint 0.5.5`，告警数不变**。无安全收益且属破坏性 major，已回滚。

| 包 | 修复前 | 修复后 |
| --- | --- | --- |
| `rustls` | 0.23.37 | **0.23.46** |
| `rustls-webpki` | 0.103.10 | **0.103.17** |
| `webpki-roots` | 1.0.6 | **1.0.12** |
| `calamine` | 0.34.0 | **0.38.0** |
| `zip` | 7.2.0 | **9.5.1** |
| `quick-xml` | 0.39.2 | **0.42.0** |
| `rkyv` | 0.7.46 | 已移除（上游不再依赖） |
| `jsonwebtoken` | 10.3.0 | 10.4.0（11.x 试验已回滚） |

### 剩余 3 条及处置判断

| Advisory | 包 | 修复版本 | 判断 |
| --- | --- | --- | --- |
| `RUSTSEC-2023-0071` | `rsa 0.9.10` | ≥ 0.10.0 | 挂在 `jsonwebtoken 10.4.0` 的 RSA 路径。`auth.rs` 使用 `Header::default()`（HS256）+ `EncodingKey::from_secret`，**未使用 RSA 验签**，实际利用面有限；升 jsonwebtoken 11 无效，需上游换 rsa 0.10 |
| `RUSTSEC-2026-0387` | `crypto-bigint 0.5.5` | ≥ 0.7.6 | 经 `elliptic-curve 0.13 → ecdsa → p256/p384 → jsonwebtoken` 引入；同上，依赖 `elliptic-curve` 大版本升级 |
| `RUSTSEC-2026-0459` | `libsqlite3-sys 0.30.1` | ≥ 0.39.0 | `sqlx 0.8.6` 的 SQLite 后端构建期依赖；项目只启用 `sqlx-postgres`，运行时不使用 SQLite。清除需 sea-orm 2 / sqlx 0.9 级升级 |

> 三条均属**上游 major 才能修**的传递依赖，且不在当前业务主路径上。策略：保留在报告中由 `rust-audit`（报告模式）持续可见，等上游发布兼容版本后再评估，不为清零而做破坏性升级。

## 5. CI 门禁策略

`security.yml` 采用「**存量可见化 + 阻断新增**」。npm 存量清零后已升级为硬门禁：

| Job | 触发 | 行为 |
| --- | --- | --- |
| `secret-scan` | push / PR / 每周 | gitleaks 扫描全历史与工作区；发现疑似真实凭据**阻断合并**；占位符由 `.gitleaks.toml` 允许列表排除 |
| `rust-audit` | push / PR / 每周 | `cargo audit` 生成 JSON 报告并上传为 artifact、写入 job summary；**报告模式**（剩余 3 条存量），不阻断 |
| `npm-audit` | push / PR / 每周 | 生成报告 + `npm audit --audit-level=high`；**high 及以上阻断合并**（2026-11-14 存量清零后启用） |
| `dependency-review` | 仅 PR | `actions/dependency-review-action` 对**本次 PR 新增/变更**的依赖做审查，`high` 及以上阻断 |

升级路径：Cargo 剩余 3 条清零后，把 `rust-audit` 也改为阻断模式；clippy 告警清零后启用 `-- -D warnings`。

## 6. 复现命令

```bash
# 全历史敏感串扫描
for c in $(git rev-list --all); do
  git grep -InE 'postgres://[^:]+:[^@]+@|BEGIN .*PRIVATE KEY|ghp_[A-Za-z0-9]{20,}|sk-[A-Za-z0-9]{20,}' "$c" --
done | sort -u

# 逐提交确认 .env 不存在
for c in $(git rev-list main); do git ls-tree -r --name-only "$c" | grep -qx '.env' && echo "LEAK $c"; done

# 已跟踪文件中的敏感路径
git ls-files | grep -Ei '\.env|\.log|logs/|node_modules|dist/|target/|\.pem|\.key|\.dump'

# 依赖审计（cargo audit 需联网拉取 RustSec advisory-db；
# 网络不通时可用 --no-fetch 读取本地缓存，结果可能略旧）
cargo audit --no-fetch
cd erp-frontend && npm audit
```

## 7. 修复优先级

| 级别 | 事项 | 说明 |
| --- | --- | --- |
| **P0** | K-07 `JWT_SECRET` 改环境变量注入并轮换已签发令牌 | 公开仓库下任何人可伪造管理员令牌；改完同步 `.env.example` 与 [SECURITY.md](SECURITY.md) |
| **P0** | K-06 轮换 PostgreSQL 口令 | 历史曾明文出现，即使已清洗也按已泄露处理；改完更新本地 `.env` |
| **P1** | K-08 默认口令 `123456` → 随机一次性口令 + 强制首次修改 | |
| **P1** | ~~升级 npm 依赖~~ / ~~跟进 calamine、rustls RUSTSEC~~ | ✅ 2026-11-14 已完成，见 §3 / §4 |
| **P2** | 跟踪剩余 3 条 RUSTSEC | 等上游提供 `rsa 0.10` / `elliptic-curve` / `libsqlite3-sys 0.39` 兼容版本；不做破坏性 major |
| **P2** | K-09 审计日志脱敏、设置保留期 | |
| **P2** | CORS 白名单、限流、令牌吊销 | 公网部署前置条件 |
| **P2** | 引入数据库迁移工具、补齐自动化测试 | 见 [architecture.md](architecture.md#9-已知架构约束与演进方向) |

> 依赖版本变动后请重新执行 [§6](#6-复现命令) 并更新本报告的数据与日期。

## 8. 拉取请求与依赖策略变更（2026-11-14）

首轮 Dependabot 全量检查一次性开出 **19 个 PR**，全部基于旧基线 `32bc751`（当前 `main` 已推进），其中含 6+ 个破坏性 major（`sea-orm 2.0.4`、`vite 8.3.8`、`jsonwebtoken 11.1.0`、`argon2 0.6.0` 等）与工具链漂移（`rust 1.99`、`node 25`、`actions/checkout 7`、`setup-node 7`）。逐个 rebase + 等 CI 的成本高，且多数在固定 Rust 1.98.0 / Node 22 的仓库里会直接红灯。

处理方式：

- 19 个 PR **统一关闭**（每个留一条说明评论），19 个来源分支全部删除，远端只保留 `main`；
- 安全更新**不是被丢弃**：`rustls 0.23.46`、`rustls-webpki 0.103.17`、`source-map-js 1.2.2`、`vue 3.5.45` 等修复已由本次统一依赖升级覆盖，且一次性覆盖了全部前端告警与大部分后端告警；
- `calamine 0.38.0` 单独 PR 的目标版本与本次升级一致，但由本地验证编译后一次性落地；
- `dependabot.yml` 收敛为：**每月一次、每个生态最多 3 个 PR、minor/patch 分组、屏蔽 `semver-major`**。后续 major 升级（含基础镜像 major tag）需维护者手工评估，不再自动开 PR。

> 结论：开放 PR 数 **19 → 0**，npm 告警 **10 → 0**，Cargo 告警 **20 → 3**。
