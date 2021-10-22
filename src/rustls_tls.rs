use fantoccini::ClientBuilder;
use hyper::client::connect::Connect;

/// Type alias for the [`Connect`] impl used in [`ClientBuilder::rustls`].
pub type Connector = hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>;

/// Type alias for using [`super::Manager`] with [`ClientBuilder::rustls`].
pub type Manager = super::Manager<Connector>;

/// Type alias for using [`super::Pool`] with [`ClientBuilder::rustls`].
pub type Pool = super::Pool<Connector>;

/// Type alias for using [`super::PoolBuilder`] with [`ClientBuilder::rustls`].
pub type PoolBuilder = super::PoolBuilder<Connector>;

/// Type alias for using [`super::BuildError`] with [`ClientBuilder::rustls`].
pub type BuildError = super::BuildError;

/// Type alias for using [`super::CreatePoolError`] with [`ClientBuilder::rustls`].
pub type CreatePoolError = super::CreatePoolError;

/// Type alias for using [`super::PoolError`] with [`ClientBuilder::rustls`].
pub type PoolError = super::PoolError;

/// Type alias for using [`super::Object`] with [`ClientBuilder::rustls`].
pub type Object = super::Object<Connector>;

/// Type alias for using [`super::Hook`] with [`ClientBuilder::rustls`].
pub type Hook = super::Hook<Connector>;

/// Type alias for using [`super::HookError`] with [`ClientBuilder::rustls`].
pub type HookError = super::HookError<Connector>;

/// Type alias for using [`super::HookErrorCause`] with [`ClientBuilder::rustls`].
pub type HookErrorCause = super::HookErrorCause<Connector>;
