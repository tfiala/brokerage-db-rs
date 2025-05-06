[<img alt="github" src="https://img.shields.io/badge/github-tfiala/brokerage-db-rs?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/tfiala/brokerage-db-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/brokerage-db.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/brokerage-db)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=docs.rs" height="20">](https://docs.rs/brokerage-db/latest/brokerage-db)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/tfiala/brokerage-db-rs/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/tfiala/brokerage-db-rs/actions/workflows/rust.yml)
[<img alt="codecov.io" src="https://img.shields.io/codecov/c/github/tfiala/brokerage-db-rs?style=for-the-badge" height="20">](https://codecov.io/gh/tfiala/brokerage-db-rs)

Database management for trader brokerage data with a MongoDB backend.


## Setup

```toml
[dependencies]
brokerage-db = "0.1.0"
```

## Functionality

Coming soon.

## How to use

```rust
use anyhow::Result;
use brokerage_db;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
```

## Roadmap

* [ ] Import brokerage data from InteractiveBrokers (IBKR) Flex-based report queries.
