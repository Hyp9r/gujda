use gujda::adapters::inbound::http::router::create_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let router = create_router();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!(
        "Server listening on http://{}",
        listener.local_addr()?
    );

    axum::serve(listener, router).await
}