#[tokio::main]
async fn main() {
    env_logger::init();
    lookup::lookup_candidates_domains().await;
}