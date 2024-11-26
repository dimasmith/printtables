use printtables::server::startup::start_server;
use sqlx::SqlitePool;
use std::future::IntoFuture;
use tokio::net::TcpListener;

pub mod project;

// Common code to handle printtables server
pub struct TestServer {
    pub port: u16,
}

pub async fn start_test_server() -> anyhow::Result<TestServer> {
    let test_listener = TcpListener::bind("0.0.0.0:0").await?;
    let port = &test_listener.local_addr()?.port();
    let db_pool = test_database_pool().await?;
    let server = start_server(test_listener, db_pool);
    tokio::spawn(server.into_future());
    Ok(TestServer { port: *port })
}

async fn test_database_pool() -> anyhow::Result<SqlitePool> {
    let database_name = uuid::Uuid::now_v7();
    let db_url = format!("file:{}.db?mode=memory&cache=shared", database_name);
    SqlitePool::connect(&db_url)
        .await
        .map_err(|e| anyhow::Error::new(e))
}
impl TestServer {
    pub fn uri(&self, path: &str) -> String {
        if !path.starts_with("/") {
            panic!("invalid path {}. path must start with /", path);
        }
        format!("http://localhost:{}{}", self.port, path)
    }
}
