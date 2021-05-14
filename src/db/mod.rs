use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

pub mod users_db;

lazy_static! {
    static ref POOL: Pool<PostgresConnectionManager> = create_pool();
}

fn create_pool() -> Pool<PostgresConnectionManager> {
    let manager =
        PostgresConnectionManager::new("postgres://rsapi@localhost:5432/rsapi", TlsMode::None)
            .expect("Unable to connect to database");
    ;
    Pool::builder()
        .max_size(20)
        .build(manager)
        .expect("Error creating DB pool")
}

pub fn get_connection() -> PooledConnection<PostgresConnectionManager> {
    POOL.get().expect("Error getting connection from pool")
}
