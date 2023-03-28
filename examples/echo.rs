use async_stdin::recv_from_stdin;

// Run with `cargo run --example echo`
// Then start sending input to stdin to see it echoed back
#[tokio::main]
async fn main() {
    let mut rx = recv_from_stdin(10);
    while let Some(s) = rx.recv().await {
        println!("Received: {}", s);
    }
}
