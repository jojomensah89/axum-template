#![allow(unused)]

use nasa_rust_implementation::run;
#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();
    run().await;
}
