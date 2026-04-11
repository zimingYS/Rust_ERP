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