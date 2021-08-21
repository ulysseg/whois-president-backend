#[tokio::main]
async fn main() {
    generate_domains::update_candidates_with_generated_domains().await;
}
