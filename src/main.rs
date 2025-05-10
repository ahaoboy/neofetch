
#[tokio::main]
async fn main() {
    // let _guard = if std::env::args().any(|arg| arg == "--no-trace") {
    //     None
    // } else {
    //     let (chrome_layer, guard) = tracing_chrome::ChromeLayerBuilder::new()
    //         .include_args(true)
    //         .build();
    //     tracing_subscriber::registry().with(chrome_layer).init();
    //     Some(guard)
    // };

    let s = neofetch::neofetch().await;
    println!("{s}")
}
