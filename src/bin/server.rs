use printtables::server::startup::start_server;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

const DEFAULT_PORT: u16 = 4229;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), DEFAULT_PORT);
    let listener = TcpListener::bind(addr).await?;
    start_server(listener).await
}
