pub mod cli;
pub mod domain;
pub mod proxy;

#[tokio::main]
async fn main() {
    cli::run().await.expect("panic message");
}
