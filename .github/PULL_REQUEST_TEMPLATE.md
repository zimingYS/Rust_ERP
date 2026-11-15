## 关联 Issue

Closes #

## 变更类型

- [ ] 🐛 缺陷修复
- [ ] ✨ 新功能
- [ ] 📝 文档
- [ ] ♻️ 重构（不改变外部行为）
- [ ] 🔧 CI / 构建 / 依赖
- [ ] 🔒 安全修复（若为未公开漏洞，请走 SECURITY.md 私有通道，不要开 PR）

## 改了什么

<!-- 先用一句话概括，再说明关键实现与取舍。涉及金额、库存、权限的改动请重点说明。 -->

## 如何验证

<!-- 执行的命令、手动步骤、前后对比截图 -->

## 提交前自检

- [ ] `cargo fmt --all -- --check` 通过
- [ ] `cargo check --workspace --all-targets` 通过
- [ ] `cargo test --workspace` 通过
- [ ] `cd erp-frontend && npm ci && npm run build` 通过
- [ ] 命名与格式符合 `docs/CODE_STYLE.md`，未引入新的 clippy 告警
- [ ] 接口 / 表结构变更已同步 `docs/api.md` / `docs/database.md`，并在 `docs/README.md` 登记新增文档
- [ ] 容器相关改动已本地验证 `docker compose config`
- [ ] 未提交 `.env`、口令、令牌、私钥、日志或数据库转储
- [ ] 新增环境变量已写入 `.env.example` 占位值
- [ ] 前端依赖变更已同步提交 `package-lock.json`，后端依赖变更已提交 `Cargo.lock`
- [ ] `README.md` / `docs/CHANGELOG.md` 已按需更新
- [ ] 我有权按 PolyForm Noncommercial 1.0.0 授权所提交的内容

---

> 主分支受保护：PR 需 `backend` 与 `frontend` 两项 CI 全绿、所有 review 对话已解决；单人维护者仓库不要求人工审批（GitHub 禁止作者批准自己的 PR）。禁止直接推送主分支。
