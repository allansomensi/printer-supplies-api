#[tokio::main]
async fn main() {
    adapters::http::run().await.unwrap();
}
