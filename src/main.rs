#[tokio::main]
async fn main() {
    let neofetch = neofetch::Neofetch::new().await;
    println!("{neofetch}")
}
