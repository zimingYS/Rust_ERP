use sea_orm::DatabaseConnection;

pub(crate) struct AppState {
    pub(crate) db: DatabaseConnection,
}
