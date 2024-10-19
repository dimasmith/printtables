use std::future::IntoFuture;
use tokio::net::TcpListener;
use printtables::server::routes::router::router;

// Common code to handle printtables server
pub struct TestServer {
    pub port: u16,
}

pub async fn start_test_server() -> anyhow::Result<TestServer> {
    let test_listener = TcpListener::bind("0.0.0.0:0").await?;
    let port = &test_listener.local_addr()?.port();
    let app_router = router();
    let test_server = axum::serve(test_listener, app_router);
    tokio::spawn(test_server.into_future());
    Ok(TestServer { port: *port })
}

impl TestServer {
    pub fn uri(&self, path: &str) -> String {
        if !path.starts_with("/") {
            panic!("invalid path {}. path must start with /", path);
        }
        format!("http://localhost:{}{}", self.port, path)
    }
}