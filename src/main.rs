use gujda::{adapters::inbound::http::router::create_router, infrastructure::logging};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    logging::init();

    let router = create_router();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let address = listener.local_addr()?;

    info!(%address, "Server started");

    axum::serve(listener, router).await
}