// API 端点常量
export const API_ENDPOINTS = {
  // 认证
  LOGIN: '/api/login',
  REGISTER: '/api/register',

  // 产品管理
  ITEMS: '/api/items',

  // 采购管理
  PURCHASE_ORDERS: '/api/purchase-orders',

  // 销售管理
  SALES_ORDERS: '/api/sales-orders',

  // 库存管理
  INVENTORY_ITEMS: '/api/inventory/items',
  INVENTORY_STATS: '/api/inventory/stats',
  INVENTORY_ADJUST: (id) => `/api/inventory/items/${id}/adjust`,
  INVENTORY_HISTORY: (id) => `/api/inventory/items/${id}/history`,
  LOW_STOCK_ALERTS: '/api/inventory/alerts/low-stock',
  REORDER_SUGGESTIONS: '/api/inventory/reorder-suggestions',
  CHECK_ALERTS: '/api/inventory/check-alerts',
  CREATE_REORDER_NOTIFICATIONS: '/api/inventory/create-reorder-notifications',
  SAFETY_STOCK: (id) => `/api/inventory/items/${id}/safety-stock`,

  // 财务管理
  FINANCE_TRANSACTIONS: '/api/finance/transactions',
  FINANCE_STATS: '/api/finance/stats',

  // 仪表盘
  DASHBOARD_STATS: '/api/dashboard/stats',
  DASHBOARD_SALES_TREND: '/api/dashboard/sales-trend',
  DASHBOARD_PURCHASE_TREND: '/api/dashboard/purchase-trend',
  DASHBOARD_INVENTORY_DIST: '/api/dashboard/inventory-distribution',
  DASHBOARD_TOP_PRODUCTS: '/api/dashboard/top-products',
  DASHBOARD_FINANCIAL_SUMMARY: '/api/dashboard/financial-summary',

  // 工作流
  WORKFLOW_START: '/api/workflow/start',
  WORKFLOW_APPROVE: (stepId) => `/api/workflow/${stepId}/approve`,
  WORKFLOW_MY_PENDING: '/api/workflow/my-pending',
  WORKFLOW_HISTORY: (businessType, businessId) => `/api/workflow/history/${businessType}/${businessId}`,

  // 数据导入
  IMPORT_PRODUCTS: '/api/import/products',
  IMPORT_SUPPLIERS: '/api/import/suppliers',
  IMPORT_TEMPLATE: '/api/import/template',

  // 消息通知
  NOTIFICATIONS: '/api/notifications',
  NOTIFICATION_STATS: '/api/notifications/stats',
  NOTIFICATION_UNREAD_COUNT: '/api/notifications/unread-count',
  NOTIFICATION_MARK_READ: (id) => `/api/notifications/${id}/read`,
  NOTIFICATION_MARK_ALL_READ: '/api/notifications/mark-all-read',

  // 供应商管理
  SUPPLIERS: '/api/suppliers',

  // 客户管理
  CUSTOMERS: '/api/customers',

  // 用户管理
  USERS: '/api/users',

  // 审计日志
  AUDIT_LOGS: '/api/audit/logs',

  // 报表导出
  REPORT_EXPORT: '/api/reports/export',
}

// 分页常量
export const PAGINATION = {
  DEFAULT_PAGE_SIZE: 10,
  PAGE_SIZES: [10, 20, 50, 100],
}

// 库存状态
export const STOCK_STATUS = {
  NORMAL: '正常',
  LOW: '低库存',
  OUT_OF_STOCK: '缺货',
}

// 订单状态
export const ORDER_STATUS = {
  PENDING_APPROVAL: '待审批',
  APPROVED: '已审批',
  CANCELLED: '已取消',
  PENDING_CONFIRM: '待确认',
  CONFIRMED: '已确认',
  COMPLETED: '已完成',
}

// 调整类型
export const ADJUSTMENT_TYPES = [
  { label: '入库', value: '入库' },
  { label: '出库', value: '出库' },
  { label: '盘点调整', value: '盘点调整' },
  { label: '损耗', value: '损耗' },
  { label: '退货', value: '退货' },
]

// 角色 ID
export const ROLES = {
  ADMIN: 1,
  SUPERVISOR: 2,
  USER: 3,
}

// 通知类型
export const NOTIFICATION_TYPES = {
  INVENTORY_ALERT: '库存预警',
  REORDER_SUGGESTION: '补货建议',
  ORDER_APPROVAL: '订单审批',
  SYSTEM: '系统通知',
}

// 通知优先级
export const NOTIFICATION_PRIORITY = {
  HIGH: '高',
  MEDIUM: '中',
  LOW: '低',
}

// 时间常量（毫秒）
export const TIME_CONSTANTS = {
  POLLING_INTERVAL: 30000, // 30秒
  DEBOUNCE_DELAY: 300, // 300毫秒
  DIALOG_CLOSE_DELAY: 300, // 300毫秒
  ONE_MINUTE_MS: 60000,
  ONE_HOUR_MS: 3600000,
  ONE_DAY_MS: 86400000,
  SEVEN_DAYS_MS: 604800000,
}

// UI常量
export const UI_CONSTANTS = {
  ICON_SIZE_SMALL: 24,
  ICON_SIZE_MEDIUM: 30,
}