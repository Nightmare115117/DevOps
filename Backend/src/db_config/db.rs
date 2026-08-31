use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct AppState {
    db: sqlx::PgPool
}

impl AppState {

    pub fn new(pool: PgPool) -> AppState{
        AppState {
            db: pool
        }
    }

    pub fn get_pool (&self) -> &PgPool {
        return &self.db;
    }

}