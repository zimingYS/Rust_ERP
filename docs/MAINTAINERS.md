# 维护者手册：公开、分支保护与发版

本文件给仓库维护者使用，记录把 `Rust_ERP` 从私有仓库转为公开协作仓库的完整操作顺序。
**顺序很重要**：先清洗 → 再公开 → 再设保护 → 最后发版。

## 0. 当前状态

| 项 | 状态 |
| --- | --- |
| 本地工程化文件（治理文档、模板、CI、发版工作流、忽略规则） | ✅ 已落地 |
| Git 历史中误提交的 `.env` | ✅ 已从全部提交中移除 |
| 38 个未提交的 rustfmt 文件 | ✅ 已纳入发布提交 |
| 版本对齐 `1.0.0` + 本地标签 `v1.0.0` | ✅ 已完成 |
| 推送清洗后的 `master` 与 `v1.0.0` 到远端 | ✅ 已完成（仓库**仍为私有**） |
| 轮换 PostgreSQL 数据库口令 | ⬜ **你来做，见 Step 0** |
| 仓库公开 / 分支保护 / 发布 Release | ⬜ **你来做，见 Step 2 ~ 6** |

> 免费版 GitHub 个人账号：**分支保护规则仅在公开仓库可用**，所以必须先执行 Step 4（公开），再执行 Step 5（保护）。两者之间不要推送任何敏感内容。

---

## Step 0 · 轮换数据库口令（公开前必做）

历史中的口令曾以明文形式存在，即使已清洗也应视为**已泄露**。先轮换，再公开。

```sql
-- 在 psql 中执行
ALTER USER postgres WITH PASSWORD '换成新的强口令';
```

然后修改本地 `.env`（该文件已被 `.gitignore` 忽略，不会提交）：

```
DATABASE_URL=postgres://postgres:新的强口令@localhost:5432/erp_db
```

同时提醒：**`JWT_SECRET` 目前是源码常量**（见 [SECURITY.md](../SECURITY.md) S-01），修复前不要把服务暴露到公网。这是代码改动，本文档不代为实施。

---

## Step 1 · 核对清洗结果

在网页端打开仓库前，先本地确认历史里已无 `.env`：

```bash
git log --all --oneline -- .env        # 应无输出
git ls-tree -r --name-only HEAD | grep -i '^\.env$'   # 应无输出
git status --short                    # .env 不应出现在未跟踪列表（已被忽略）
```

远端确认：`https://github.com/zimingYS/Rust_ERP/commits/master` 逐个提交不应包含 `.env`，且标签列表里应有 `v1.0.0`。

---

## Step 2 · 补全仓库信息（可在私有状态完成）

`Settings → General`：

| 字段 | 建议值 |
| --- | --- |
| Description | `基于 Rust (Axum + SeaORM) 与 Vue 3 的 ERP 系统，毕业设计作品 · 可分享、禁止商业使用` |
| Website | 留空，或填 GitHub Pages / 演示地址 |
| Topics | `erp` `rust` `axum` `sea-orm` `vue3` `vite` `postgresql` `graduation-project` |
| Features | 勾选 Issues；**开启 Discussions**（建议） |
| Pull Requests | 勾选 `Allow squash merging`；勾选 `Automatically delete head branches`；可关闭 `Allow merge commits` 以保持线性历史 |

> 许可文件 `LICENSE` 是 PolyForm Noncommercial 1.0.0，GitHub 会识别为 `Other`，这是预期行为（非 OSI 许可证）。

---

## Step 3 · 等待首次 CI 全绿

清洗后的提交已推送，`Actions → CI` 会自动跑一次。确认两个 job 都是绿色：

- `backend`：fmt / check / test / clippy（报告模式）
- `frontend`：npm ci / npm run build

**这两个名字就是后面要勾选的"必需状态检查"**。没跑过一次的话，保护规则里搜不到它们。

---

## Step 4 · 设为公开

`Settings → General` 拉到最底部 `Danger Zone` → `Change visibility` → `Make public`。

GitHub 会要求输入仓库全名确认。确认前再核对一次 Step 0 与 Step 1。

**公开后立刻执行 Step 5，中间不要停。**

---

## Step 5 · 保护主分支（核心）

路径：`Settings → Branches → Add branch protection rule`
（如果界面默认进入新版 Rulesets，找 `Add classic branch protection rule` 链接，下面按经典规则描述。）

| 配置项 | 值 | 作用 |
| --- | --- | --- |
| Branch name pattern | `master` | 保护主分支 |
| ✅ Require a pull request before merging | 开启 | **禁止所有人直接推送**，外部提交只能走 PR |
| └ Required approvals | `1` | 至少 1 人批准 |
| └ Dismiss stale pull request approvals | 开启 | 新提交会作废旧批准 |
| └ Require review from Code Owners | 开启 | 由 `.github/CODEOWNERS` 指定的 `@zimingYS` 审批 |
| └ Require approval of the most recent reviewable push | 建议开启 | 防止审批后夹带提交 |
| ✅ Require status checks to pass before merging | 开启 | CI 不绿不能合并 |
| └ Require branches to be up to date | 开启 | 合并前必须与 master 同步 |
| └ 搜索并添加检查项 | `backend`、`frontend` | 对应 CI 的两个 job |
| ✅ Require conversation resolution before merging | 开启 | 所有 review 对话必须解决 |
| ✅ Require linear history | 可选（配合 squash 合并建议开启） | 保持历史整洁 |
| ✅ Do not allow bypassing the above settings | 开启 | **管理员也受约束**（含 Include administrators 语义） |
| ❌ Allow force pushes | **不勾** | 防止覆盖历史 |
| ❌ Allow deletions | **不勾** | 防止误删主分支 |
| Restrict who can push | 可选，填 `zimingYS` | 视账号套餐是否提供 |

点 `Create` / `Save changes` 保存。

### 验证保护是否生效

```bash
# 期望：远程拒绝（protected branch hook declined）
git checkout master && git commit --allow-empty -m "test: 验证保护" && git push origin master
```

验证用的空提交记得本地 `git reset --hard HEAD~1` 撤销，**不要留在 master 上**。

### 他人如何"能提交"

- 仓库公开后，**任何人都可以 Fork 并开 PR**，无需加协作者、无需授予写权限 —— 这就是"允许提交但进不了主分支"的标准形态。
- 贡献者提交后，CI 自动运行，你作为 Code Owner 审批合并。
- 若确实需要给某人直接推分支的权限：`Settings → Collaborators → Add people`，给 `Write`；仍建议对方走 PR。

---

## Step 6 · 发布第一版 Release

`Releases → Draft a new release`：

| 字段 | 值 |
| --- | --- |
| Choose a tag | 选择已存在的 `v1.0.0` |
| Release title | `v1.0.0 — 首个公开发行版` |
| Set as latest release | ✅ |
| Set as pre-release | ❌（你选的是正式 v1.0.0） |

描述可直接基于 `CHANGELOG.md` 的 `1.0.0` 段落，建议附加一句：

> 本项目采用 PolyForm Noncommercial 1.0.0：可自由学习与分享，禁止商业使用。
> 当前版本存在已记录的安全限制（见 SECURITY.md），请勿部署到公网或用于生产环境。

点击 **Publish release**。随后 `Release Assets` 工作流会自动触发，构建并附加：
`erp-backend-linux-x86_64-v1.0.0.tar.gz`、`erp-frontend-dist-v1.0.0.tar.gz`、`env.example`、`SHA256SUMS.txt`。
在 `Actions` 里确认该工作流成功，再回到 Release 页面核对附件齐全。

---

## Step 7 · 可选收尾

### 标签体系（模板里已引用，建议建齐）

`Issues → Labels → New label`：

| 标签 | 颜色 | 用途 |
| --- | --- | --- |
| `bug` | `#d73a4a` | 缺陷 |
| `enhancement` | `#a2eeef` | 功能建议 |
| `triage` | `#fbca04` | 待分类 |
| `documentation` | `#0075ca` | 文档 |
| `good first issue` | `#7057ff` | 适合新贡献者 |
| `help wanted` | `#008672` | 需要协助 |
| `dependencies` | `#0366d6` | Dependabot 依赖升级 |
| `backend` / `frontend` / `ci` | `#c5def5` / `#bfd4f2` / `#fef2c0` | 影响范围 |
| `security` | `#b60205` | 已公开的安全事项 |

### 把 `master` 改名为 `main`（可选）

`Settings → Branches` 的分支重命名按钮会自动更新默认分支、打开的 PR 与保护规则。
改完后需同步修改：
- `.github/workflows/ci.yml` 的 `branches: [master, main]`（已同时列出，可不改）
- `.github/ISSUE_TEMPLATE/config.yml` 里的文档链接
- 所有贡献者的本地远端跟踪分支：`git branch -m master main && git fetch origin && git branch -u origin/main main`

> **建议：v1.0.0 已按 `master` 发布，本轮先不改名**，等下一个版本再统一迁移，避免刚发版就制造混乱。

---

## 附录 A · 想改主意用 API 一键完成

本机已有具备 `repo` / `workflow` / `admin` 权限的凭据时，可用以下命令替代 Step 2/4/5/6（`OWNER=zimingYS`、`REPO=Rust_ERP`、`TOKEN` 自行准备）：

```bash
# 公开 + 仓库设置
curl -X PATCH -H "Authorization: Bearer $TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/zimingYS/Rust_ERP \
  -d '{"private":false,"has_discussions":true,"delete_branch_on_merge":true,"allow_squash_merge":true}'

# 分支保护
curl -X PUT -H "Authorization: Bearer $TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/zimingYS/Rust_ERP/branches/master/protection \
  -d '{"required_status_checks":{"strict":true,"contexts":["backend","frontend"]},"enforce_admins":true,"required_pull_request_reviews":{"dismiss_stale_reviews":true,"require_code_owner_reviews":true,"required_approving_review_count":1},"restrictions":null,"required_linear_history":false,"allow_force_pushes":false,"allow_deletions":false,"required_conversation_resolution":true}'

# 发布 Release
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/zimingYS/Rust_ERP/releases \
  -d '{"tag_name":"v1.0.0","name":"v1.0.0 — 首个公开发行版","body":"见 CHANGELOG.md","draft":false,"prerelease":false}'
```

注意：API 设置 `required_status_checks.contexts` 前，CI 仍需至少成功运行过一次，否则检查项不会被识别。

---

## 附录 B · 后续发版流程

1. 在 PR 中完成变更并通过保护规则合并到 `master`。
2. 更新 `CHANGELOG.md`（`Unreleased` → 新版本段落）。
3. 同步四处版本号：`Cargo.toml`、`erp-backend/Cargo.toml`、`erp-frontend/package.json`、`erp-frontend/package-lock.json`，并提交 `Cargo.lock`。
4. 打标签并推送：`git tag -a v1.0.1 -m "..." && git push origin v1.0.1`。
5. 在 Releases 页面基于该标签 Publish release，产物工作流自动附加构建文件。
6. 若 CI job 改名，同步更新保护规则中的必需状态检查。

## 附录 C · 回滚

若公开后发现敏感信息残留：

1. 立刻 `Settings → General → Danger Zone → Change visibility → Make private`（先止血）
2. 轮换所有泄露的凭据
3. 参考清洗脚本重写历史并强推（**强推前先临时关闭保护规则**）
4. 重新确认无误后再公开，并恢复保护规则
