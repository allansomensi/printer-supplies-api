mod handlers;
mod models;
mod router;
mod server;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    println!("🌟 Printer Supplies API 🌟");

    server::run().await.unwrap();
}
