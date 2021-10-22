# deadpool-fantoccini

[![CI](https://github.com/OpenByteDev/deadpool-fantoccini/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/deadpool-fantoccini/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/deadpool-fantoccini.svg)](https://crates.io/crates/deadpool-fantoccini)
[![Documentation](https://docs.rs/deadpool-fantoccini/badge.svg)](https://docs.rs/deadpool-fantoccini)
[![dependency status](https://deps.rs/repo/github/openbytedev/deadpool-fantoccini/status.svg)](https://deps.rs/repo/github/openbytedev/deadpool-fantoccini)
[![MIT](https://img.shields.io/crates/l/deadpool-fantoccini.svg)](https://github.com/OpenByteDev/deadpool-fantoccini/blob/master/LICENSE)

[`deadpool`](https://crates.io/crates/deadpool/0.9.0) is a dead simple async pool for connections and objects of any type.

This crate implements a deadpool manager for [`fantoccini`](https://crates.io/crates/fantoccini/0.17.6).

## Example
```rust
use deadpool_fantoccini::{Manager, Pool, PoolShutdown};
use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
async fn main() {
    let manager = Manager::new("http://localhost:4444", ClientBuilder::native());
    let pool = Pool::builder(manager).max_size(5).build().unwrap();

    let mut client = pool.get().await.unwrap();
    client.goto("http://example.org/").await.unwrap();
    let title = client
        .find(Locator::Css("h1"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(title, "Example Domain");
    drop(client);

    // cleanly closes all sessions (all sessions have to be returned to the pool beforehand.)
    pool.shutdown().await.unwrap();
}
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/deadpool-fantoccini/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
