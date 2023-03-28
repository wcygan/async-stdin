# Async Stdin

[<img alt="github" src="https://img.shields.io/badge/github-wcygan/async--stdin-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcygan/async-stdin)
[<img alt="crates.io" src="https://img.shields.io/crates/v/async-stdin.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/async-stdin)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-async--stdin-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/async-stdin)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcygan/async-stdin/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/wcygan/async-stdin/actions?query=branch%3Amain)

Read from stdin over a Tokio channel

This is useful for interactive programs that read from stdin while waiting for other events to occur.

# Usage

Add this to your Cargo.toml:

```toml
[dependencies]
async-stdin = "0.3.1"
```

You can read from stdin like so:

```rust
use async_stdin::recv_from_stdin;

#[tokio::main]
async fn main() {
    let mut rx = recv_from_stdin(10);
    while let Some(s) = rx.recv().await {
        println!("Received: {}", s);
    }
}
```