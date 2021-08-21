#[tokio::main]
async fn main() {
    env_logger::init();
    lookup::perform().await;
}