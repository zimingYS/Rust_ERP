pub const ROLE_ADMIN: i32 = 1;
pub const ROLE_SUPERVISOR: i32 = 2;
pub const ROLE_USER: i32 = 3;

pub const ORDER_STATUS_PENDING_APPROVAL: &str = "待审批";
pub const ORDER_STATUS_APPROVED: &str = "已审批";
pub const ORDER_STATUS_CANCELLED: &str = "已取消";

pub const SALES_ORDER_STATUS_PENDING_CONFIRM: &str = "待确认";
pub const SALES_ORDER_STATUS_CONFIRMED: &str = "已确认";
pub const SALES_ORDER_STATUS_COMPLETED: &str = "已完成";

pub const ADJUSTMENT_TYPE_INBOUND: &str = "入库";
pub const ADJUSTMENT_TYPE_OUTBOUND: &str = "出库";
pub const ADJUSTMENT_TYPE_COUNTING: &str = "盘点调整";
pub const ADJUSTMENT_TYPE_LOSS: &str = "损耗";
pub const ADJUSTMENT_TYPE_RETURN: &str = "退货";

pub const PURCHASE_ORDER_PREFIX: &str = "PO";
pub const SALES_ORDER_PREFIX: &str = "SO";

// 分页常量
pub const DEFAULT_PAGE_SIZE: u64 = 10;
pub const MAX_PAGE_SIZE: u64 = 100;
// pub const MIN_PAGE_SIZE: u64 = 1;
//
// // 库存常量
// pub const DEFAULT_STOCK: i32 = 0;
// pub const DEFAULT_MIN_STOCK: i32 = 0;

// 工作流常量
pub const WORKFLOW_STATUS_PENDING: &str = "待处理";
pub const WORKFLOW_STATUS_WAITING: &str = "等待中";
pub const WORKFLOW_STATUS_APPROVED: &str = "已通过";
pub const WORKFLOW_STATUS_REJECTED: &str = "已拒绝";
pub const WORKFLOW_STATUS_COMPLETED: &str = "已完成";
// pub const WORKFLOW_STATUS_IN_PROGRESS: &str = "进行中";

// 采购提前期（天）
pub const DEFAULT_LEAD_TIME_DAYS: i32 = 7;

// 安全库存计算参数
pub const SERVICE_FACTOR_95: f64 = 1.65; // 95%服务水平的Z值
pub const INVENTORY_DAYS_BASE: f64 = 30.0; // 库存天数基准
pub const REORDER_DAYS_TARGET: f64 = 60.0; // 补货目标天数

// 消息通知优先级
pub const PRIORITY_HIGH: &str = "高";
pub const PRIORITY_MEDIUM: &str = "中";
pub const PRIORITY_LOW: &str = "低";

// JWT Token 有效期（秒）
pub const TOKEN_EXPIRY_SECONDS: usize = 86400; // 24小时

// 仪表盘趋势天数
pub const TREND_DAYS: i64 = 7;

// 补货计算倍数
pub const REORDER_MULTIPLIER: i32 = 2;

// 客户默认评分
pub const DEFAULT_CUSTOMER_RATING: i32 = 3;

// 通知类型
pub const NOTIFICATION_TYPE_INVENTORY_ALERT: &str = "库存预警";
pub const NOTIFICATION_TYPE_REORDER_SUGGESTION: &str = "补货建议";
// pub const NOTIFICATION_TYPE_ORDER_APPROVAL: &str = "订单审批";
// pub const NOTIFICATION_TYPE_SYSTEM: &str = "系统通知";
