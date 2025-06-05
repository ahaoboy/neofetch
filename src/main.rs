#[tokio::main]
async fn main() {
    let s = neofetch::neofetch().await;
    println!("{s}")
}
