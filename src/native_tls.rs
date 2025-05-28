#[allow(unused_imports)]
use fantoccini::ClientBuilder;
use hyper_util::client::legacy::connect::HttpConnector;

/// Type alias for the [`Connect`] impl used in [`ClientBuilder::native`].
pub type Connector = hyper_tls::HttpsConnector<HttpConnector>;

/// Type alias for using [`super::Manager`] with [`ClientBuilder::native`].
pub type Manager = super::Manager<Connector>;

/// Type alias for using [`super::Pool`] with [`ClientBuilder::native`].
pub type Pool = super::Pool<Connector>;

/// Type alias for using [`super::PoolBuilder`] with [`ClientBuilder::native`].
pub type PoolBuilder = super::PoolBuilder<Connector>;

/// Type alias for using [`super::BuildError`] with [`ClientBuilder::native`].
pub type BuildError = super::BuildError;

/// Type alias for using [`super::CreatePoolError`] with [`ClientBuilder::native`].
pub type CreatePoolError = super::CreatePoolError;

/// Type alias for using [`super::PoolError`] with [`ClientBuilder::native`].
pub type PoolError = super::PoolError;

/// Type alias for using [`super::Object`] with [`ClientBuilder::native`].
pub type Object = super::Object<Connector>;

/// Type alias for using [`super::Hook`] with [`ClientBuilder::native`].
pub type Hook = super::Hook<Connector>;

/// Type alias for using [`super::HookError`] with [`ClientBuilder::native`].
pub type HookError = super::HookError<Connector>;
