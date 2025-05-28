#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
#![deny(
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![forbid(unsafe_code)]
#![warn(
    non_ascii_idents,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    clippy::pedantic,
    clippy::cargo,
    unsafe_op_in_unsafe_fn
)]
#![allow(clippy::cargo_common_metadata, clippy::no_effect_underscore_binding)]

/*!
[`deadpool`] is a dead simple async pool for connections and objects of any type.

This crate implements a deadpool manager for [`fantoccini`].

## Example
```rust
use deadpool_fantoccini::{Manager, Pool, PoolShutdown};
use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
async fn main() {
    # let webdriver_install_dir = tempfile::Builder::new()
    #     .prefix("deadpool-fantoccini-test-webdriver-install-")
    #     .tempdir()
    #     .unwrap();
    # let webdriver_install_path = webdriver_install_dir.path().to_path_buf();
    # defer_lite::defer! {
    #    webdriver_install_dir.close().unwrap();
    # };
    #
    # let webdriver_path = tokio::task::spawn_blocking(move || {
    #     webdriver_install::Driver::Chrome
    #         .install_into(webdriver_install_path)
    #         .unwrap()
    # })
    # .await
    # .unwrap();
    #
    # let mut webdriver = std::process::Command::new(&webdriver_path)
    #     .arg("--port=4444")
    #     .spawn()
    #     .unwrap();
    # defer_lite::defer! {
    #    webdriver.kill().unwrap();
    #    webdriver.wait().unwrap();
    # };
    #
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

    // cleanly close all sessions (all sessions have to be returned to the pool beforehand)
    pool.shutdown().await.unwrap();
}
```
!*/

#[cfg(any(feature = "native-tls", feature = "doc_cfg"))]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "native-tls")))]
/// Type aliases for using this crate with [`native-tls`](https://crates.io/crates/native-tls).
pub mod native_tls;

#[cfg(any(feature = "rustls-tls", feature = "doc_cfg"))]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "rustls-tls")))]
/// Type aliases for using this crate with [`rustls`](https://crates.io/crates/rustls).
pub mod rustls_tls;

/// Reexports of the dependencies of this crate.
pub mod reexports;

/// Type alias for using [`deadpool::managed::Pool`] with [`fantoccini`].
pub type Pool<C> = deadpool::managed::Pool<Manager<C>, deadpool::managed::Object<Manager<C>>>;

/// Type alias for using [`deadpool::managed::PoolBuilder`] with [`fantoccini`].
pub type PoolBuilder<C> =
    deadpool::managed::PoolBuilder<Manager<C>, deadpool::managed::Object<Manager<C>>>;

/// Type alias for using [`deadpool::managed::BuildError`] with [`fantoccini`].
pub type BuildError = deadpool::managed::BuildError;

/// Type alias for using [`deadpool::managed::CreatePoolError`] with [`fantoccini`].
pub type CreatePoolError = deadpool::managed::CreatePoolError<
    fantoccini::error::NewSessionError
>;

/// Type alias for using [`deadpool::managed::PoolError`] with [`fantoccini`].
pub type PoolError = deadpool::managed::PoolError<fantoccini::error::NewSessionError>;

/// Type alias for using [`deadpool::managed::Object`] with [`fantoccini`].
pub type Object<C> = deadpool::managed::Object<Manager<C>>;

/// Type alias for using [`deadpool::managed::Hook`] with [`fantoccini`].
pub type Hook<C> = deadpool::managed::Hook<Manager<C>>;

/// Type alias for using [`deadpool::managed::HookError`] with [`fantoccini`].
pub type HookError<C> = deadpool::managed::HookError<Manager<C>>;

use hyper_util::client::legacy::connect::Connect;

/// [`deadpool::managed::Manager`] for creating and recycling [`fantoccini::Client`]s.
#[derive(Debug, Clone)]
pub struct Manager<C>
where
    C: 'static + Connect + Send + Sync + Clone + Unpin,
{
    client_builder: fantoccini::ClientBuilder<C>,
    webdriver_url: String,
}

impl<C> Manager<C>
where
    C: 'static + Connect + Send + Sync + Clone + Unpin,
{
    /// Creates a new [`Manager`] using the given webdriver url and the [`fantoccini::ClientBuilder`] used to contruct new connections.
    pub fn new(
        webdriver_url: impl Into<String>,
        client_builder: fantoccini::ClientBuilder<C>,
    ) -> Self {
        Self {
            client_builder,
            webdriver_url: webdriver_url.into(),
        }
    }

    /// Gets the url of the webdriver instance used to construct sessions.
    pub fn webdriver_url(&self) -> &str {
        &self.webdriver_url
    }

    /// Gets the [`fantoccini::ClientBuilder`] used to contruct new connections.
    pub fn session_builder(&self) -> &fantoccini::ClientBuilder<C> {
        &self.client_builder
    }
}

impl<C> deadpool::managed::Manager for Manager<C>
where
    C: 'static + Connect + Send + Sync + Clone + Unpin,
{
    type Type = fantoccini::Client;
    type Error = fantoccini::error::NewSessionError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        self.client_builder
            .connect(self.webdriver_url.as_str())
            .await
    }

    async fn recycle(
        &self,
        _obj: &mut Self::Type,
        _metrics: &deadpool::managed::Metrics,
    ) -> deadpool::managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

/// Extensions trait used to provide a way to cleanly close all active sessions from a [`Pool`].
pub trait PoolShutdown {
    /// Error that the [`Pool`](`deadpool::managed::Pool`) can return when closing an active session from the pool.
    type Error;

    /// Cleanly close all active sessions from this pool.
    /// New sessions should not be created while this method is running.
    fn shutdown(self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}
 
impl<C> PoolShutdown for Pool<C>
where
    C: 'static + Connect + Send + Sync + Clone + Unpin,
{
    type Error = fantoccini::error::CmdError;

    async fn shutdown(self) -> Result<(), Self::Error> {
        while self.status().size > 0 {
            match self.get().await {
                Ok(o) => {
                    self.resize(self.status().max_size - 1);
                    deadpool::managed::Object::take(o).close().await?;
                }
                Err(e) => panic!("{e:#?}"),
            }
        }

        // a session could be created here, which would not be closed cleanly.

        self.close();

        Ok(())
    }
}
