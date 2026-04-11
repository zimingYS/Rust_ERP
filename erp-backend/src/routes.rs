use axum::{middleware, routing::get, Router};
use std::sync::Arc;
use axum::routing::post;
use crate::handlers::{item_handler, health, user_handler, purchase_handler, sales_handler, inventory_handler, finance_handler, audit_handler, report_handler};
use crate::middleware::auth_middleware;
use crate::state::AppState;

pub fn create_router(state: Arc<AppState>) -> Router{
    let protected_routes = Router::new()
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
        
        .route("/api/inventory",
               get(inventory_handler::get_inventory_list)
        )
        .route("/api/inventory/stats",
               get(inventory_handler::get_inventory_stats)
        )
        .route("/api/inventory/{id}",
               get(inventory_handler::get_item_inventory)
        )
        .route("/api/inventory/{id}/adjust",
               post(inventory_handler::adjust_stock)
        )
        .route("/api/inventory/{id}/history",
               get(inventory_handler::get_adjustment_history)
        )
        .route("/api/inventory/batch-adjust",
               post(inventory_handler::batch_adjust_stock)
        )

        .route("/api/finance/transactions",
               get(finance_handler::get_transactions)
                   .post(finance_handler::create_transaction)
        )
        .route("/api/finance/stats",
               get(finance_handler::get_financial_stats)
        )
        .route("/api/finance/category-summary/{transaction_type}",
               get(finance_handler::get_category_summary)
        )
        .route("/api/finance/transactions/{id}",
               axum::routing::delete(finance_handler::delete_transaction)
        )
        .route("/api/finance/auto/purchase/{order_id}",
               post(finance_handler::auto_create_purchase_transaction)
        )
        .route("/api/finance/auto/sales/{order_id}",
               post(finance_handler::auto_create_sales_transaction)
        )

        .route("/api/audit/logs",
               get(audit_handler::get_audit_logs)
                   .post(audit_handler::create_audit_log)
        )
        .route("/api/audit/activity-summary",
               get(audit_handler::get_user_activity_summary)
        )

        .route("/api/reports/export/purchase-orders",
               get(report_handler::export_purchase_orders)
        )
        .route("/api/reports/export/sales-orders",
               get(report_handler::export_sales_orders)
        )
        .route("/api/reports/export/inventory",
               get(report_handler::export_inventory)
        )
        .route("/api/reports/export/financial-transactions",
               get(report_handler::export_financial_transactions)
        )
        .route("/api/reports/export/audit-logs",
               get(report_handler::export_audit_logs)
        )

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

        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .route("/api/health", get(health::health_check))

        .route("/api/register", post(user_handler::register))
        .route("/api/login", post(user_handler::login))

        .merge(protected_routes)
        .with_state(state)
}