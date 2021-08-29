use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(generate_domains::update_candidates_with_generated_domains().await?)
}
