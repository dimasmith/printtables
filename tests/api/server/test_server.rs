use anyhow::anyhow;
use printtables::server::startup::start_server;
use reqwest::Client;
use sqlx::SqlitePool;
use std::future::IntoFuture;
use tokio::net::TcpListener;
// Common code to handle printtables server
pub struct TestServer {
    pub port: u16,
    pub api_client: Client,
}

pub async fn start_test_server() -> anyhow::Result<TestServer> {
    let test_listener = TcpListener::bind("0.0.0.0:0").await?;
    let port = &test_listener.local_addr()?.port();
    let db_pool = test_database_pool().await?;
    let server = start_server(test_listener, db_pool);
    tokio::spawn(server.into_future());
    let api_client = Client::new();
    Ok(TestServer {
        port: *port,
        api_client,
    })
}

async fn test_database_pool() -> anyhow::Result<SqlitePool> {
    let database_name = uuid::Uuid::now_v7();
    let db_url = format!("file:{}.db?mode=memory&cache=shared", database_name);
    SqlitePool::connect(&db_url).await.map_err(|e| anyhow!(e))
}
impl TestServer {
    pub fn uri(&self, path: &str) -> String {
        if !path.starts_with("/") {
            panic!("invalid path {}. path must start with /", path);
        }
        format!("http://localhost:{}{}", self.port, path)
    }
}
