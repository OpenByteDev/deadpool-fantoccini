#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results
)]

/*!
Deadpool is a dead simple async pool for connections and objects of any type.

This crate implements a deadpool manager for [`fantoccini`].

# Example
```rust
use deadpool_fantoccini::{Manager, Pool, PoolShutdown};
use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
async fn main() {
    let manager = Manager::new("http://localhost:4444", fantoccini::ClientBuilder::native());
    let pool = Pool::builder(manager)
        .max_size(5)
        .build()
        .unwrap();
    
    let mut client = pool.get().await.unwrap();
    client.goto("http://example.org/").await.unwrap();
    let title = client.find(Locator::Css("h1")).await.unwrap().text().await.unwrap();
    assert_eq!(title, "Example Domain");
    drop(client);

    // cleanly closes all sessions (all sessions have to be returned to the pool beforehand.)
    pool.shutdown().await.unwrap();
}
```
!*/

use deadpool::async_trait;

/// Type alias for the [`hyper::client::connect::Connect`] used as default in [`fantoccini`].
pub type Connector = hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>;

/// Type alias for using [`raw::Manager`] for the default connectors.
pub type Manager = raw::Manager<Connector>;

/// Type alias for using [`deadpool::managed::Pool`] with [`fantoccini`] with one of the default connectors.
pub type Pool = raw::Pool<Connector>;

/// Type alias for using [`deadpool::managed::PoolBuilder`] with [`fantoccini`] with one of the default connectors.
pub type PoolBuilder = raw::PoolBuilder<Connector>;

/// Type alias for using [`deadpool::managed::BuildError`] with [`fantoccini`] with one of the default connectors.
pub type BuildError = raw::BuildError;

/// Type alias for using [`deadpool::managed::CreatePoolError`] with [`fantoccini`] with one of the default connectors.
pub type CreatePoolError = raw::CreatePoolError;

/// Type alias for using [`deadpool::managed::PoolError`] with [`fantoccini`] with one of the default connectors.
pub type PoolError = raw::PoolError;

/// Type alias for using [`deadpool::managed::Object`] with [`fantoccini`] with one of the default connectors.
pub type Object = raw::Object<Connector>;

/// Type alias for using [`deadpool::managed::Hook`] with [`fantoccini`] with one of the default connectors.
pub type Hook = raw::Hook<Connector>;

/// Type alias for using [`deadpool::managed::HookError`] with [`fantoccini`] with one of the default connectors.
pub type HookError = raw::HookError<Connector>;

/// Type alias for using [`deadpool::managed::HookErrorCause`] with [`fantoccini`] with one of the default connectors.
pub type HookErrorCause = raw::HookErrorCause<Connector>;

/// [`deadpool::managed::Manager`] implementation and type aliases for using custom [`hyper::client::connect::Connect`] implementations.
pub mod raw {
    use deadpool::async_trait;

    /// Type alias for using [`deadpool::managed::Pool`] with [`fantoccini`].
    pub type Pool<C> = deadpool::managed::Pool<Manager<C>, deadpool::managed::Object<Manager<C>>>;

    /// Type alias for using [`deadpool::managed::PoolBuilder`] with [`fantoccini`].
    pub type PoolBuilder<C> = deadpool::managed::PoolBuilder<Manager<C>, deadpool::managed::Object<Manager<C>>>;

    /// Type alias for using [`deadpool::managed::BuildError`] with [`fantoccini`].
    pub type BuildError = deadpool::managed::BuildError<fantoccini::error::NewSessionError>;

    /// Type alias for using [`deadpool::managed::CreatePoolError`] with [`fantoccini`].
    pub type CreatePoolError = deadpool::managed::CreatePoolError<std::convert::Infallible, fantoccini::error::NewSessionError>;

    /// Type alias for using [`deadpool::managed::PoolError`] with [`fantoccini`].
    pub type PoolError = deadpool::managed::PoolError<fantoccini::error::NewSessionError>;

    /// Type alias for using [`deadpool::managed::Object`] with [`fantoccini`].
    pub type Object<C> = deadpool::managed::Object<Manager<C>>;

    /// Type alias for using [`deadpool::managed::Hook`] with [`fantoccini`].
    pub type Hook<C> = deadpool::managed::Hook<Manager<C>>;

    /// Type alias for using [`deadpool::managed::HookError`] with [`fantoccini`].
    pub type HookError<C> = deadpool::managed::HookError<Manager<C>>;

    /// Type alias for using [`deadpool::managed::HookErrorCause`] with [`fantoccini`].
    pub type HookErrorCause<C> = deadpool::managed::HookErrorCause<Manager<C>>;

    /// [`Manager`] for creating and recycling [`fantoccini::Client`]s.
    #[derive(Debug, Clone)]
    pub struct Manager<C>
    where
        C: 'static + hyper::client::connect::Connect + Send + Sync + Clone + Unpin,
    {
        client_builder: fantoccini::ClientBuilder<C>,
        webdriver_url: String,
    }

    impl<C> Manager<C>
    where
        C: 'static + hyper::client::connect::Connect + Send + Sync + Clone + Unpin,
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

    #[async_trait]
    impl<C> deadpool::managed::Manager for Manager<C>
    where
        C: 'static + hyper::client::connect::Connect + Send + Sync + Clone + Unpin,
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
        ) -> deadpool::managed::RecycleResult<Self::Error> {
            Ok(())
        }
    }
}

/// Extensions trait used to provide a way to cleanly close all active sessions from a [`Pool`].
#[async_trait]
pub trait PoolShutdown {
    /// Error that the [`Pool`] can return when closing an active session from the pool.
    /// [`Pool`]: deadpool::managed::Pool
    type Error;

    /// Cleanly close all active sessions from this pool.
    /// New sessions should be created while this method is running.
    async fn shutdown(self) -> Result<(), Self::Error>;
}

#[async_trait]
impl <C> PoolShutdown for raw::Pool<C>
where C: 'static + hyper::client::connect::Connect + Send + Sync + Clone + Unpin {
    type Error = fantoccini::error::CmdError;

    async fn shutdown(self) -> Result<(), Self::Error> {
        while self.status().size > 0 {
            match self.get().await {
                Ok(o) => {
                    self.resize(self.status().max_size - 1);
                    deadpool::managed::Object::take(o).close().await?;
                }
                Err(e) => panic!("{:#?}", e),
            }
        }

        // a session could be created here which would not be closed cleanly.

        self.close();

        Ok(())
    }
}
