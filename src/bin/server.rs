use printtables::server::startup::start_server;
use sqlx::SqlitePool;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

const DEFAULT_PORT: u16 = 4229;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), DEFAULT_PORT);
    let listener = TcpListener::bind(addr).await?;

    let db_pool = dev_database_pool().await?;

    start_server(listener, db_pool).await
}

async fn dev_database_pool() -> anyhow::Result<SqlitePool> {
    let db_url = dotenvy::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:dev.db".to_string());
    SqlitePool::connect(&db_url)
        .await
        .map_err(anyhow::Error::new)
}
