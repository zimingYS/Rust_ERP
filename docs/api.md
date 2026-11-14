# API 接口文档

> 依据 `erp-backend/src/routes.rs`（唯一路由装配点）逐条整理，覆盖 **v1.0.0 的全部 `/api/*` 端点**。请求 / 响应体的结构以各 `handlers/*.rs` 中的 `Json` 提取器与 `serde` 结构体为准。

## 1. 基础约定

| 项 | 值 |
| --- | --- |
| Base URL（本地） | `http://127.0.0.1:3000` |
| 协议 | HTTP/1.1 + JSON（`Content-Type: application/json`） |
| 路径前缀 | `/api` |
| 认证 | `Authorization: Bearer <token>`，由 `POST /api/login` 签发 |
| 令牌 | HS256，`UserClaims { user_id, username, role_id, exp }`，有效期 86400 秒 |
| 权限层级 | **公开** / **登录用户** / **管理员**（`role_id == 1`） |
| 分页 | 查询参数 `page`（默认 1）、`page_size`（默认 10，最大 100）；列表响应含 `pages` 等元数据 |
| 错误 | `AppError` 统一转换，前端按 `response.data.error` 读取；常见码：`400 / 401 / 403 / 404 / 500` |
| CORS | `allow_origin(Any)` + `allow_methods(Any)` + `allow_headers(Any)`（见 S-03） |

健康检查返回**纯文本** `ERP 系统运行正常！`，不是 JSON。

### 权限分组实现

| 组 | 定义位置 | 中间件 |
| --- | --- | --- |
| 公开 | `Router::new()` 顶层三条 | 无 |
| `admin_routes` | `routes.rs:15-42` | `auth_middleware` + `require_admin_middleware` |
| `protected_routes` | `routes.rs:44` 起 | `auth_middleware` |

> ⚠️ 所有 `/api/*` 请求还会经过全局 `audit_logger_middleware`，响应后写入 `audit_logs`。

## 2. 公开接口

| 方法 | 路径 | 说明 | 成功响应 |
| --- | --- | --- | --- |
| `GET` | `/api/health` | 健康检查（静态字符串，不探活数据库） | `200` 纯文本 |
| `POST` | `/api/register` | 注册账号，argon2 哈希口令 | `200 {"msg":"注册成功"}` |
| `POST` | `/api/login` | 登录并签发 JWT | `200 {"token":"..."}` |

## 3. 管理员接口

需要有效 JWT **且** `role_id == 1`；否则 `401`（无 / 无效令牌）或 `403`（角色不足）。

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/api/users` | 用户列表 |
| `POST` | `/api/users` | 新建用户 |
| `GET` | `/api/users/roles` | 角色列表 |
| `GET` | `/api/users/{id}` | 用户详情 |
| `PUT` | `/api/users/{id}` | 更新用户（含角色） |
| `DELETE` | `/api/users/{id}` | 删除用户 |
| `POST` | `/api/users/{id}/change-password` | 修改口令 |
| `POST` | `/api/users/{id}/reset-password` | 重置口令（⚠️ 当前重置为 `123456`，见 S-02） |
| `GET` | `/api/audit/logs` | 审计日志查询 |
| `POST` | `/api/audit/logs` | 写入审计日志 |
| `GET` | `/api/audit/activity-summary` | 用户活动汇总 |

## 4. 登录用户接口

### 4.1 客户 / 供应商

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` / `POST` | `/api/customers` | 客户列表 / 新建 |
| `GET` | `/api/customers/stats` | 客户统计 |
| `GET` / `PUT` / `DELETE` | `/api/customers/{id}` | 详情 / 更新 / 删除 |
| `GET` / `POST` | `/api/suppliers` | 供应商列表 / 新建 |
| `GET` | `/api/suppliers/stats` | 供应商统计 |
| `GET` / `PUT` / `DELETE` | `/api/suppliers/{id}` | 详情 / 更新 / 删除 |

### 4.2 物品与库存

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` / `POST` | `/api/items` | 物品列表 / 新建 |
| `DELETE` | `/api/items` | **批量删除**（注意：DELETE 打在集合路径上，不是 `/api/items/{id}`） |
| `GET` / `PUT` / `DELETE` | `/api/items/{id}` | 详情 / 更新 / 删除 |
| `POST` | `/api/items/{id}/stock` | 调整库存 |
| `GET` | `/api/inventory/items` | 库存列表 |
| `GET` | `/api/inventory/items/{id}` | 单品库存详情 |
| `POST` | `/api/inventory/items/{id}/adjust` | 库存调整（写 `inventory_adjustments`） |
| `GET` | `/api/inventory/items/{id}/history` | 调整历史 |
| `GET` | `/api/inventory/items/{id}/safety-stock` | 自动计算安全库存 |
| `GET` | `/api/inventory/stats` | 库存统计 |
| `GET` | `/api/inventory/alerts/low-stock` | 低库存预警列表 |
| `GET` | `/api/inventory/reorder-suggestions` | 补货建议 |
| `POST` | `/api/inventory/check-alerts` | 主动检查并生成预警 |
| `POST` | `/api/inventory/create-reorder-notifications` | 生成补货通知 |

### 4.3 采购订单

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` / `POST` | `/api/purchase-orders` | 订单列表 / 新建（含明细） |
| `GET` | `/api/purchase-orders/stats` | 订单统计 |
| `GET` / `PUT` / `DELETE` | `/api/purchase-orders/{id}` | 详情（含明细）/ 更新 / 删除 |
| `POST` | `/api/purchase-orders/{id}/approve` | 审批通过 |
| `POST` | `/api/purchase-orders/{id}/cancel` | 取消 |
| `DELETE` | `/api/purchase-orders/force-delete/{id}` | 强制删除 |
| `GET` | `/api/purchase-orders/suppliers` | 下单用供应商下拉数据 |

### 4.4 销售订单

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` / `POST` | `/api/sales-orders` | 订单列表 / 新建（含明细） |
| `GET` | `/api/sales-orders/stats` | 订单统计 |
| `GET` / `PUT` / `DELETE` | `/api/sales-orders/{id}` | 详情（含明细）/ 更新 / 删除 |
| `POST` | `/api/sales-orders/{id}/confirm` | 确认 |
| `POST` | `/api/sales-orders/{id}/complete` | 完成 |
| `POST` | `/api/sales-orders/{id}/cancel` | 取消 |
| `DELETE` | `/api/sales-orders/force-delete/{id}` | 强制删除 |
| `GET` | `/api/sales-orders/customers` | 下单用客户下拉数据 |

### 4.5 财务 / 报表 / 通知

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` / `POST` | `/api/transactions` | 收支流水列表 / 新建 |
| `GET` | `/api/transactions/stats` | 财务统计 |
| `GET` | `/api/transactions/categories` | 分类汇总 |
| `DELETE` | `/api/transactions/{id}` | 删除流水 |
| `GET` | `/api/reports/export/purchase` | 导出采购订单 CSV |
| `GET` | `/api/reports/export/sales` | 导出销售订单 CSV |
| `GET` | `/api/reports/export/inventory` | 导出库存 CSV |
| `GET` | `/api/reports/export/finance` | 导出财务流水 CSV |
| `GET` | `/api/reports/export/audit` | 导出审计日志 CSV |
| `GET` / `POST` | `/api/notifications` | 通知列表 / 新建 |
| `GET` | `/api/notifications/stats` | 通知统计 |
| `GET` | `/api/notifications/unread-count` | 未读数量 |
| `POST` | `/api/notifications/{id}/read` | 标记单条已读 |
| `POST` | `/api/notifications/mark-all-read` | 全部标记已读 |
| `DELETE` | `/api/notifications/{id}` | 删除通知 |

### 4.6 仪表盘

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/api/dashboard/stats` | 概览指标 |
| `GET` | `/api/dashboard/sales-trend` | 销售趋势（近 7 天） |
| `GET` | `/api/dashboard/purchase-trend` | 采购趋势 |
| `GET` | `/api/dashboard/inventory-distribution` | 库存分布 |
| `GET` | `/api/dashboard/top-products` | 热销物品 |
| `GET` | `/api/dashboard/financial-summary` | 财务摘要 |

### 4.7 工作流 / 导入

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `POST` | `/api/workflow/start` | 发起流程（写实例 + 步骤） |
| `POST` | `/api/workflow/{step_id}/approve` | 审批步骤 |
| `GET` | `/api/workflow/my-pending` | 我的待办 |
| `GET` | `/api/workflow/history/{business_type}/{business_id}` | 流程历史 |
| `POST` | `/api/import/products` | 导入物品（Excel / CSV，`multipart`） |
| `POST` | `/api/import/suppliers` | 导入供应商 |
| `GET` | `/api/import/template` | 下载导入模板 |

## 5. 典型调用

```bash
# 登录
curl -s -X POST http://127.0.0.1:3000/api/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"demo","password":"..."}'
# -> {"token":"eyJ..."}

# 带令牌查询物品（分页）
curl -s http://127.0.0.1:3000/api/items?page=1\&page_size=20 \
  -H 'Authorization: Bearer eyJ...'

# 健康检查
curl -s http://127.0.0.1:3000/api/health
```

前端统一通过 `erp-frontend/src/utils/request.js` 发请求：请求拦截器注入 Bearer，响应拦截器直接返回 `response.data`，并对 401 清理 `localStorage.erp_token` 后跳转 `/login`。

## 6. 已知前后端契约偏差

`erp-frontend/src/api/constants.js` 中的常量与后端实际路由存在以下不一致，**以 `routes.rs` 为准**：

| 前端常量 | 前端值 | 后端实际 | 影响 |
| --- | --- | --- | --- |
| `FINANCE_TRANSACTIONS` | `/api/finance/transactions` | `/api/transactions` | 按常量直接请求会 404 |
| `FINANCE_STATS` | `/api/finance/stats` | `/api/transactions/stats` | 同上 |
| `REPORT_EXPORT` | `/api/reports/export` | `/api/reports/export/{purchase\|sales\|inventory\|finance\|audit}` | 是前缀而非完整端点，需拼接具体类型 |

此外：

- 前端 `ORDER_STATUS`、`ADJUSTMENT_TYPES`、`ROLES`、`NOTIFICATION_TYPES` 与 `constants.rs` 的中文取值目前一致，但属于**两处手工维护**，易漂移。
- 部分后端端点（`/api/transactions/categories`、订单状态流转、导入模板等）在 `constants.js` 中没有对应常量，页面内可能直接写字符串。
- 修复建议：把前端常量统一改为与后端一致，或由后端下发端点字典；改动需同步更新本文与 [modules.md](modules.md)。

> 这些是**现状记录**，本文档不代为修改业务代码。

## 7. 已知接口设计问题

| 编号 | 问题 | 建议 |
| --- | --- | --- |
| P-01 | `force-delete/{id}` 挂在 `protected_routes`，任何登录用户可调用 | 移入 `admin_routes` |
| P-02 | `DELETE /api/items` 承担批量删除，与 REST 集合语义不符 | 改为 `POST /api/items/batch-delete` |
| P-03 | 无统一分页响应结构，各 handler 自行组装 | 定义 `Page<T>` 泛型 |
| P-04 | 错误响应结构未在 OpenAPI 中声明，无机器可读契约 | 引入 `utoipa` 生成 OpenAPI |
| P-05 | 认证仅靠固定 `JWT_SECRET`，无刷新 / 吊销机制 | 环境变量注入 + 黑名单 |
| P-06 | 导出接口直接返回 CSV，无异步任务与大数据量保护 | 加分页 / 异步导出 |
