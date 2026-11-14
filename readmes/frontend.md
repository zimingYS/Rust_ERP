# 前端模块速查

面向「我要改前端页面或接口调用」的场景。完整接口清单见 [docs/api.md](../docs/api.md)，规范见 [docs/CODE_STYLE.md](../docs/CODE_STYLE.md)。

## 结构

```
erp-frontend/src/
├── main.js                  # 挂载 Vue App、Router、Element Plus
├── App.vue
├── router/index.js          # 路由表 + beforeEach 登录守卫
├── utils/request.js         # 唯一 Axios 实例：baseURL / 拦截器 / 错误处理
├── api/constants.js         # 接口路径与业务枚举常量
├── components/Layout.vue    # 侧边栏 + 顶栏的登录态外壳
├── views/                   # 13 个页面组件
└── style.css / assets/
```

## 页面与路由

| 路由 | 组件 | 说明 |
| --- | --- | --- |
| `/login`、`/register` | `Login.vue`、`Register.vue` | 公开 |
| `/home` | `HomeDashboard.vue` | 仪表盘（趋势、分布、Top 产品） |
| `/products` | `ProductManagement.vue` | 物品 / SKU |
| `/purchase` | `PurchaseManagement.vue` | 采购订单 |
| `/sales` | `SalesManagement.vue` | 销售订单 |
| `/suppliers`、`/customers` | `SupplierManagement.vue`、`CustomerManagement.vue` | 基础资料 |
| `/inventory` | `InventoryManagement.vue` | 库存、预警、补货 |
| `/finance` | `FinanceManagement.vue` | 收支流水 |
| `/reports` | `ReportExport.vue` | 报表导出 |
| `/audit` | `AuditLog.vue` | 审计日志 |
| `/users` | `UserManagement.vue` | 用户与角色（管理员） |

父路由 `/` 使用 `Layout` 并带 `meta.requiresAuth`，守卫只检查 `localStorage.erp_token` **是否存在**，不校验过期；过期由 Axios 的 401 拦截器清理并跳转登录。

## 请求链路

```
页面 → utils/request.js (Axios 实例)
     → 请求拦截器注入 Authorization: Bearer <erp_token>
     → http://127.0.0.1:3000 + 路径
     → 响应拦截器直接返回 response.data；401/400/500 统一 ElMessage 提示
```

**联调要点**

- `baseURL` 在 `utils/request.js:6` 写死为 `http://127.0.0.1:3000`，Vite 没有配代理。改后端地址、走容器或反向代理时必须改这里（建议改为 `import.meta.env.VITE_API_BASE_URL ?? ...`）。
- `api/constants.js` 是路径常量的**期望**集合，与后端 `routes.rs` 有偏差，已知项见 [docs/api.md](../docs/api.md#6-已知前后端契约偏差)。
- 错误体约定 `{ "error": "..." }`，响应拦截器读取 `error.response.data.error`。
- 导入 / 导出接口用 `multipart` 或文件下载，不要复用普通 JSON 的假设。

## 自检

```bash
cd erp-frontend
npm ci
npm run build
npm audit
```

> 前端目前**没有** ESLint / Prettier 门禁；`.editorconfig` 与 `docs/CODE_STYLE.md` 定义了约定，格式化工具有待引入，详见规范文档。
