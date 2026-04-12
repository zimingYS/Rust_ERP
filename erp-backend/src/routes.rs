use axum::routing::get;
use axum::{middleware, Router};
use std::sync::Arc;
use axum::routing::post;
use crate::handlers::{item_handler, health, user_handler, purchase_handler, sales_handler, inventory_handler, finance_handler, audit_handler, report_handler, user_management_handler, supplier_handler, customer_handler, notification_handler, dashboard_handler, workflow_handler};
use crate::middleware::{auth_middleware, require_admin_middleware};
use crate::state::AppState;

pub fn create_router(state: Arc<AppState>) -> Router{
    // 管理员专属路由
    let admin_routes = Router::new()
        .route("/api/users",
               get(user_management_handler::get_users)
                   .post(user_management_handler::create_user)
        )
        .route("/api/users/roles",
               get(user_management_handler::get_roles)
        )
        .route("/api/users/{id}",
               axum::routing::get(user_management_handler::get_user_by_id)
                   .put(user_management_handler::update_user)
                   .delete(user_management_handler::delete_user)
        )
        .route("/api/users/{id}/change-password",
               post(user_management_handler::change_password)
        )
        .route("/api/users/{id}/reset-password",
               post(user_management_handler::reset_password)
        )
        .route("/api/audit/logs",
               get(audit_handler::get_audit_logs)
                   .post(audit_handler::create_audit_log)
        )
        .route("/api/audit/activity-summary",
               get(audit_handler::get_user_activity_summary)
        )
        .layer(middleware::from_fn(require_admin_middleware));

    // 普通用户可访问的路由
    let protected_routes = Router::new()
        // 客户管理
        .route("/api/customers",
               get(customer_handler::get_customers)
                   .post(customer_handler::create_customer)
        )
        .route("/api/customers/stats",
               get(customer_handler::get_customer_stats)
        )
        .route("/api/customers/{id}",
               axum::routing::get(customer_handler::get_customer_by_id)
                   .put(customer_handler::update_customer)
                   .delete(customer_handler::delete_customer)
        )

        // 供应商管理
        .route("/api/suppliers",
               get(supplier_handler::get_suppliers)
                   .post(supplier_handler::create_supplier)
        )
        .route("/api/suppliers/stats",
               get(supplier_handler::get_supplier_stats)
        )
        .route("/api/suppliers/{id}",
               axum::routing::get(supplier_handler::get_supplier_by_id)
                   .put(supplier_handler::update_supplier)
                   .delete(supplier_handler::delete_supplier)
        )

        // 物料管理
        .route("/api/items",
            get(item_handler::get_items)
                .post(item_handler::create_item)
                .delete(item_handler::batch_delete_items)
        )
        .route("/api/items/{id}",
            get(item_handler::get_item_by_id)
                .put(item_handler::update_item)
                .delete(item_handler::delete_item)
        )
        .route("/api/items/{id}/stock",
            post(item_handler::adjust_stock)
        )
        
        // 库存管理
        .route("/api/inventory/items",
               get(inventory_handler::get_inventory_list)
        )
        .route("/api/inventory/stats",
               get(inventory_handler::get_inventory_stats)
        )
        .route("/api/inventory/items/{id}",
               axum::routing::get(inventory_handler::get_item_inventory)
        )
        .route("/api/inventory/items/{id}/adjust",
               post(inventory_handler::adjust_stock)
        )
        .route("/api/inventory/items/{id}/history",
               get(inventory_handler::get_adjustment_history)
        )
        .route("/api/inventory/alerts/low-stock",
               get(inventory_handler::get_low_stock_alerts)
        )
        .route("/api/inventory/reorder-suggestions",
               get(inventory_handler::get_reorder_suggestions)
        )
        .route("/api/inventory/check-alerts",
               post(inventory_handler::check_and_create_alerts)
        )
        .route("/api/inventory/items/{id}/safety-stock",
               get(inventory_handler::auto_calculate_safety_stock)
        )

        // 采购管理
        .route("/api/purchase-orders",
               get(purchase_handler::get_purchase_orders)
                   .post(purchase_handler::create_purchase_order)
        )
        .route("/api/purchase-orders/stats",
               get(purchase_handler::get_purchase_order_stats)
        )
        .route("/api/purchase-orders/{id}",
               axum::routing::get(purchase_handler::get_purchase_order_with_items)
                   .put(purchase_handler::update_purchase_order)
                   .delete(purchase_handler::delete_purchase_order)
        )
        .route("/api/purchase-orders/{id}/approve",
               post(purchase_handler::approve_purchase_order)
        )
        .route("/api/purchase-orders/{id}/cancel",
               post(purchase_handler::cancel_purchase_order)
        )
        .route("/api/purchase-orders/force-delete/{id}",
               axum::routing::delete(purchase_handler::force_delete_purchase_order)
        )
        .route("/api/purchase-orders/suppliers",
               get(purchase_handler::get_suppliers_for_purchase)
        )

        // 销售管理
        .route("/api/sales-orders",
               get(sales_handler::get_sales_orders)
                   .post(sales_handler::create_sales_order)
        )
        .route("/api/sales-orders/stats",
               get(sales_handler::get_sales_order_stats)
        )
        .route("/api/sales-orders/{id}",
               axum::routing::get(sales_handler::get_sales_order_with_items)
                   .put(sales_handler::update_sales_order)
                   .delete(sales_handler::delete_sales_order)
        )
        .route("/api/sales-orders/{id}/confirm",
               post(sales_handler::confirm_sales_order)
        )
        .route("/api/sales-orders/{id}/complete",
               post(sales_handler::complete_sales_order)
        )
        .route("/api/sales-orders/{id}/cancel",
               post(sales_handler::cancel_sales_order)
        )
        .route("/api/sales-orders/force-delete/{id}",
               axum::routing::delete(sales_handler::force_delete_sales_order)
        )
        .route("/api/sales-orders/customers",
               get(sales_handler::get_customers_for_sales)
        )

        // 财务管理
        .route("/api/transactions",
               get(finance_handler::get_transactions)
                   .post(finance_handler::create_transaction)
        )
        .route("/api/transactions/stats",
               get(finance_handler::get_financial_stats)
        )
        .route("/api/transactions/categories",
               get(finance_handler::get_category_summary)
        )
        .route("/api/transactions/{id}",
               axum::routing::delete(finance_handler::delete_transaction)
        )

        // 报表导出
        .route("/api/reports/export/purchase",
               get(report_handler::export_purchase_orders)
        )
        .route("/api/reports/export/sales",
               get(report_handler::export_sales_orders)
        )
        .route("/api/reports/export/inventory",
               get(report_handler::export_inventory)
        )
        .route("/api/reports/export/finance",
               get(report_handler::export_financial_transactions)
        )
        .route("/api/reports/export/audit",
               get(report_handler::export_audit_logs)
        )

        // 消息通知
        .route("/api/notifications",
               get(notification_handler::get_user_notifications)
                   .post(notification_handler::create_notification)
        )
        .route("/api/notifications/stats",
               get(notification_handler::get_notification_stats)
        )
        .route("/api/notifications/unread-count",
               get(notification_handler::get_unread_count)
        )
        .route("/api/notifications/{id}/read",
               post(notification_handler::mark_as_read)
        )
        .route("/api/notifications/mark-all-read",
               post(notification_handler::mark_all_as_read)
        )
        .route("/api/notifications/{id}",
               axum::routing::delete(notification_handler::delete_notification)
        )

        // 仪表盘数据
        .route("/api/dashboard/stats",
               get(dashboard_handler::get_dashboard_stats)
        )
        .route("/api/dashboard/sales-trend",
               get(dashboard_handler::get_sales_trend)
        )
        .route("/api/dashboard/purchase-trend",
               get(dashboard_handler::get_purchase_trend)
        )
        .route("/api/dashboard/inventory-distribution",
               get(dashboard_handler::get_inventory_distribution)
        )
        .route("/api/dashboard/top-products",
               get(dashboard_handler::get_top_products)
        )
        .route("/api/dashboard/financial-summary",
               get(dashboard_handler::get_financial_summary)
        )

        // 工作流引擎
        .route("/api/workflow/start",
               post(workflow_handler::start_workflow)
        )
        .route("/api/workflow/{step_id}/approve",
               post(workflow_handler::approve_step)
        )
        .route("/api/workflow/my-pending",
               get(workflow_handler::get_my_pending_approvals)
        )
        .route("/api/workflow/history/{business_type}/{business_id}",
               get(workflow_handler::get_workflow_history)
        );

    Router::new()
        .route("/api/health", get(health::health_check))
        .route("/api/register", post(user_handler::register))
        .route("/api/login", post(user_handler::login))
        .merge(admin_routes.layer(middleware::from_fn(auth_middleware)))
        .merge(protected_routes.layer(middleware::from_fn(auth_middleware)))
        .with_state(state)
}