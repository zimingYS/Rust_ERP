# 维护者手册

面向仓库所有者：仓库设置、分支保护、社区运营、回滚与后续发版。发版细节见 [RELEASING.md](RELEASING.md)，安全检查见 [SECURITY_AUDIT.md](SECURITY_AUDIT.md)。

## 1. 当前状态

| 项 | 状态 |
| --- | --- |
| 默认分支 | `main`（由 `master` 重命名，本地 / 远端一致） |
| 可见性 | **Public**（免费个人账号下，分支保护仅在公开仓库可用） |
| Issues / Discussions | 均已开启，Discussion 分类模板在 `.github/DISCUSSION_TEMPLATE/` |
| 分支保护 | `main` 经典保护规则，配置见 [§3](#3-分支保护规则) |
| 许可证 | PolyForm Noncommercial 1.0.0，GitHub 显示 `Other`（非 OSI，预期行为） |
| 第一个 Release | `v1.0.0` 标签已存在；Release 由维护者在网页端 Publish |
| Git 历史 | 已重写清除误提交的 `.env`，逐提交验证无残留 |
| Dependabot 首次批量 PR | 2026-11-14 已统一关闭 **19 个**并删除全部来源分支；规则收敛为月度 minor/patch 分组，见 [§6](#6-dependabot-策略) |

> **公开后仍需处理**：轮换 PostgreSQL 口令、把 `JWT_SECRET` 改为环境变量注入、修复默认口令 `123456`。这是 [SECURITY_AUDIT.md](SECURITY_AUDIT.md) 的 P0 项，属业务源码改动，本轮未代为实施。

## 2. 仓库设置

| 字段 | 值 |
| --- | --- |
| Description | `基于 Rust (Axum + SeaORM) 与 Vue 3 的 ERP 系统，毕业设计作品 · 可分享、禁止商业使用` |
| Topics | `erp` `rust` `axum` `sea-orm` `vue3` `vite` `postgresql` `graduation-project` |
| Features | Issues ✅、Discussions ✅、Wiki ❌ |
| Pull Requests | squash merging ✅、自动删除来源分支 ✅ |
| 默认分支 | `main` |

复核：

```bash
curl -s -H "Authorization: Bearer $TOKEN" \
  https://api.github.com/repos/zimingYS/Rust_ERP \
  | jq '{private,default_branch,has_discussions,has_issues,description,topics,delete_branch_on_merge}'
```

## 3. 分支保护规则

`Settings → Branches → Classic branch protection rule`，pattern `main`。本轮通过 `PUT /repos/{owner}/{repo}/branches/main/protection` 创建，规则如下：

| 配置项 | 值 | 作用 |
| --- | --- | --- |
| Require a pull request before merging | ✅ | 禁止所有人直接推送 `main` |
| └ Required approvals | `1` | 至少 1 人批准 |
| └ Dismiss stale approvals | ✅ | 新提交作废旧批准 |
| └ Require review from Code Owners | ✅ | 由 `.github/CODEOWNERS` 指定的 `@zimingYS` 审批 |
| └ Require approval of most recent push | ✅ | 防止审批后夹带提交 |
| Require status checks to pass | ✅ | CI 不绿不能合并 |
| └ strict（分支保持最新） | ✅ | 合并前与 `main` 同步 |
| └ contexts | `backend`、`frontend` | 对应 `ci.yml` 的两个 job |
| Require conversation resolution | ✅ | review 对话必须解决 |
| Do not allow bypassing the above settings | ✅ | **管理员同样受约束** |
| Allow force pushes / deletions | ❌ | 防止覆盖历史 / 误删分支 |
| Require linear history | ❌ | 当前用 squash 合并，暂不额外强制 |
| Restrict who can push | 不设置 | 公开仓库中任何人可 Fork + 开 PR |

### 他人如何「能提交但进不了主分支」

- 公开仓库中**任何人可 Fork 并开 PR**，无需授予写权限；保护规则挡住直推 `main`。
- 贡献者 PR 自动触发 CI，Code Owner 审批后由维护者 squash 合并。
- 若需要给某人直接推分支权限：`Settings → Collaborators → Add people`（Write），仍建议走 PR。

### 验证保护生效

```bash
curl -s -H "Authorization: Bearer $TOKEN" \
  https://api.github.com/repos/zimingYS/Rust_ERP/branches/main/protection | jq .
```

不要用 `git push` 做破坏性验证；如需验证，用空提交并立刻本地 `git reset --hard HEAD~1` 撤销。

### 修改保护规则

```bash
curl -X PUT -H "Authorization: Bearer $TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/zimingYS/Rust_ERP/branches/main/protection \
  -d '{"required_status_checks":{"strict":true,"contexts":["backend","frontend"]},"enforce_admins":true,"required_pull_request_reviews":{"dismiss_stale_reviews":true,"require_code_owner_reviews":true,"required_approving_review_count":1,"require_last_push_approval":true},"restrictions":null,"required_linear_history":false,"allow_force_pushes":false,"allow_deletions":false,"required_conversation_resolution":true}'
```

> 若 CI job 改名，必须同步更新 `contexts`，否则 PR 会永远停在「等待状态检查」。

## 4. GitHub Discussions

Discussions 已开启，用于**使用咨询、设计讨论、想法交流**；缺陷与功能请求仍走 Issue 模板。

| 分类 | 模板文件 | 用途 |
| --- | --- | --- |
| Announcements | `.github/DISCUSSION_TEMPLATE/announcements.yml` | 版本发布、规则变更公告 |
| Ideas | `.github/DISCUSSION_TEMPLATE/ideas.yml` | 新想法与方案讨论 |
| Q&A | `.github/DISCUSSION_TEMPLATE/q-a.yml` | 启动 / 部署 / 使用咨询 |

维护动作：把 v1.0.0 的 Release 说明转成一则 Announcements 置顶帖，并在其中链接 [README.md](../README.md) 与 [文档索引](README.md)。`.github/ISSUE_TEMPLATE/config.yml` 已把咨询类流量导向 Discussions。

## 5. 标签体系

Issue / PR 模板引用的标签建议在 `Issues → Labels` 建齐：

| 标签 | 颜色 | 用途 |
| --- | --- | --- |
| `bug` | `#d73a4a` | 缺陷 |
| `enhancement` | `#a2eeef` | 功能建议 |
| `documentation` | `#0075ca` | 文档 |
| `triage` | `#fbca04` | 待分类 |
| `good first issue` | `#7057ff` | 适合新贡献者 |
| `help wanted` | `#008672` | 需要协助 |
| `dependencies` | `#0366d6` | Dependabot 依赖升级 |
| `backend` / `frontend` / `ci` / `docker` | `#c5def5` / `#bfd4f2` / `#fef2c0` / `#0db7ed` | 影响范围 |
| `security` | `#b60205` | 已公开的安全事项 |

## 6. Dependabot 策略

`.github/dependabot.yml` 覆盖 `cargo` / `npm` / `github-actions` / `docker` 五个条目（docker 分根目录与前端两个目录）。2026-11-14 收敛后的规则：

| 项 | 值 | 原因 |
| --- | --- | --- |
| 频率 | 每月 | 首轮全量检查一次开出 19 个 PR，噪声过大 |
| `open-pull-requests-limit` | 3 / 生态 | 同上 |
| minor / patch | 按生态分组为一个 PR | 一次 review 一批，减少 CI 轮次 |
| `version-update:semver-major` | **全部忽略** | 本仓库固定 Rust 1.98.0 / Node 22；major 会破坏编译或让 CI 红灯 |

首轮 19 个 PR 已于 2026-11-14 统一关闭（每个留说明评论）并删除来源分支，远端只剩 `main`。其安全更新由一次统一的 `chore(deps)` 提交覆盖，结果见 [SECURITY_AUDIT.md](SECURITY_AUDIT.md) §8。

**major 升级需手工评估**，建议流程：新建分支 → 改 manifest → 本地 `cargo check` / `npm run build` → 跑 `docs/SECURITY_AUDIT.md` §6 的审计命令 → 确认确有安全收益再合并。反例：本轮实测 `jsonwebtoken 11.1.0` 仍依赖 `rsa 0.9.10`，告警数不变，已回滚。

批量关闭 PR 的 API 示例（按需替换编号）：

```bash
for n in 1 2 3; do
  curl -s -X PATCH -H "Authorization: Bearer $TOKEN" \
    "https://api.github.com/repos/zimingYS/Rust_ERP/pulls/$n" \
    -d '{"state":"closed"}' -o /dev/null -w "PR #$n -> %{http_code}\n"
done
```

依赖升级带来安全告警变化时，同步刷新 [SECURITY_AUDIT.md](SECURITY_AUDIT.md) 与 [CHANGELOG.md](CHANGELOG.md)。

## 7. 后续发版

见 [RELEASING.md](RELEASING.md)：版本号同步 → tag → GHCR 镜像 / Release 产物 → 可选 SSH 部署。

## 8. 回滚预案

若公开后发现敏感信息残留：

1. 立刻改回私有止血：`PATCH /repos/zimingYS/Rust_ERP` `{"private":true}`。
2. **先轮换**所有可能泄露的凭据（数据库口令、JWT 密钥），再处理历史。
3. 重写历史需强推：先临时关闭 `main` 保护规则的 `allow_force_pushes`，推完**立即恢复**。
4. 重新确认无泄露后再公开，并复核保护规则仍完整。

若误删 / 误改保护规则：按 [§3](#3-分支保护规则) 的 API 命令重建。
