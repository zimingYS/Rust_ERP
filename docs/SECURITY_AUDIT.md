# 密钥与依赖安全检查报告

**检查日期**：2026-11-14 · **基线**：`v1.0.0` / `main` · **结论**：Git 历史中**无真实凭据残留**，但源码中仍有 **2 处硬编码凭据 / 弱口令**与 **30 项依赖安全告警**，公开仓库后属于已披露信息。

## 1. 检查方法

| 范围 | 方法 |
| --- | --- |
| Git 全历史 | 对 `git rev-list --all` 的每个提交执行 `git grep`，模式覆盖 `postgres://user:pass@`、私钥头、`password = "..."`、`ghp_`、`sk-`；并逐提交用 `git ls-tree -r --name-only` 检查 `.env` 是否存在 |
| 工作区 | `git ls-files` 按敏感路径模式过滤；扫描未跟踪的 `.env*`、`logs/`、`*.log`、`*.2026-*`、`*.bak`、数据库转储 |
| 依赖 | `npm audit --json`（`erp-frontend`）、`cargo audit --json`（workspace）实跑 |
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

> 本轮按教练模式约定**未修改任何业务源码**，K-07 / K-08 的修复由仓库所有者自行完成。

## 3. 依赖安全：npm

`npm audit` 结果：**10 项**（high 8 / moderate 2 / low 0 / critical 0），全部标记 `fixAvailable: true`。

| 依赖 | 级别 | 直接依赖 | 主要问题 |
| --- | --- | --- | --- |
| `axios` | high | ✅ | `validateStatus` 合并策略的原型污染导致认证绕过；NO_PROXY 绕过（127.0.0.0/8） |
| `vue` / `@vue/server-renderer` | high | ✅ / 间接 | SSR 属性名黑名单缺失 CR 导致 XSS |
| `vite` | high | ✅ | `launch-editor` 在 Windows 下 UNC 路径泄露 NTLMv2 哈希、参数转义不完整 |
| `echarts` | moderate | ✅ | XSS |
| `postcss` | high | 间接 | `</style>` 未转义导致 XSS；sourceMappingURL 任意文件读 |
| `source-map-js` | high | 间接 | 构造 source map 造成事件循环 DoS |
| `form-data` | high | 间接 | multipart 字段名 / 文件名 CRLF 注入 |
| `nanoid` | high | 间接 | 非安全生成器在特定参数下无限循环 |
| `follow-redirects` | moderate | 间接 | 跨域重定向泄露自定义认证头 |

## 4. 依赖安全：Cargo

`cargo audit` 结果：**20 条 RUSTSEC 告警 + 3 条 warning**（含 unmaintained / informational 类别）。代表性条目：

| Advisory | 包 | 说明 |
| --- | --- | --- |
| `RUSTSEC-2026-0436` | `calamine 0.34.0` | Excel 解析相关告警，直接影响导入接口 |
| `RUSTSEC-2023-0071` | `rsa 0.9.10` | Marvin 攻击 / 时序侧信道 |
| `RUSTSEC-2026-0285` / `0346` / `0348` / `0349` | `rustls 0.23.37` | TLS 栈多条告警（数据库连接经 rustls） |
| `RUSTSEC-2026-0098` / `0099` / `0104` / `0382` | `rustls-webpki 0.103.10` | 证书校验相关告警 |
| `RUSTSEC-2026-0194` / `0195` | `quick-xml 0.39.2` | XML 解析告警（calamine 传递依赖） |
| `RUSTSEC-2026-0235` | `rkyv 0.7.46` | 反序列化告警 |
| `RUSTSEC-2026-0387` | `crypto-bigint 0.5.5` | 密码学实现告警 |
| `RUSTSEC-2026-0459` | `libsqlite3-sys 0.30.1` | 构建期依赖告警 |

> 部分告警属于**传递依赖 / 构建期依赖**，实际利用面需逐个评估；但 `calamine`（导入入口）与 `rustls`（数据库连接）应优先跟进。

## 5. CI 门禁策略

`security.yml` 采用「**存量可见化 + 阻断新增**」，避免 30 项存量告警让 CI 长期红灯：

| Job | 触发 | 行为 |
| --- | --- | --- |
| `secret-scan` | push / PR / 每周 | gitleaks 扫描全历史与工作区；发现疑似真实凭据**阻断合并**；占位符由 `.gitleaks.toml` 允许列表排除 |
| `rust-audit` | push / PR / 每周 | `cargo audit` 生成 JSON 报告并上传为 artifact、写入 job summary；**报告模式**，不阻断 |
| `npm-audit` | push / PR / 每周 | `npm audit --json` 同上，报告模式 |
| `dependency-review` | 仅 PR | `actions/dependency-review-action` 对**本次 PR 新增/变更**的依赖做审查，`high` 及以上阻断 |

升级路径：存量告警清零后，把 `rust-audit` / `npm-audit` 改为阻断模式；clippy 告警清零后启用 `-- -D warnings`。

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

# 依赖审计
cargo audit
cd erp-frontend && npm audit
```

## 7. 修复优先级

| 级别 | 事项 | 说明 |
| --- | --- | --- |
| **P0** | K-07 `JWT_SECRET` 改环境变量注入并轮换已签发令牌 | 公开仓库下任何人可伪造管理员令牌；改完同步 `.env.example` 与 [SECURITY.md](SECURITY.md) |
| **P0** | K-06 轮换 PostgreSQL 口令 | 历史曾明文出现，即使已清洗也按已泄露处理；改完更新本地 `.env` |
| **P1** | K-08 默认口令 `123456` → 随机一次性口令 + 强制首次修改 | |
| **P1** | 升级 `axios` / `vue` / `vite` / `echarts` 到修复版本 | 需回归前端构建与页面行为；major 升级单独评估 |
| **P1** | 跟进 `calamine` / `rustls` 相关 RUSTSEC | 通过 Dependabot grouped PR 逐个验证 |
| **P2** | K-09 审计日志脱敏、设置保留期 | |
| **P2** | CORS 白名单、限流、令牌吊销 | 公网部署前置条件 |
| **P2** | 引入数据库迁移工具、补齐自动化测试 | 见 [architecture.md](architecture.md#9-已知架构约束与演进方向) |

> 依赖版本变动后请重新执行 [§6](#6-复现命令) 并更新本报告的数据与日期。
