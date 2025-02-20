use rust_service::cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
