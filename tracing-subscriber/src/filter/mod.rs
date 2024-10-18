//! [Subscribers](crate::subscribe) that control which spans and events are
//! enabled by the wrapped collector.
//!
<<<<<<< HEAD
//! This module contains a number of types that provide implementations of
//! various strategies for filtering which spans and events are enabled. For
//! details on filtering spans and events using [`Layer`]s, see the
//! [`layer` module's documentation].
//!
//! [`layer` module's documentation]: crate::layer#filtering-with-layers
//! [`Layer`]: crate::layer
mod filter_fn;

feature! {
    #![all(feature = "env-filter", feature = "std")]
    mod env;
    pub use self::env::*;
}

feature! {
    #![all(feature = "registry", feature = "std")]
    mod layer_filters;
    pub use self::layer_filters::*;
}

||||||| 386969ba
//! [`Layer`]: ../layer/trait.Layer.html
#[cfg(feature = "env-filter")]
mod env;
=======
//! This module contains a number of types that provide implementations of
//! various strategies for filtering which spans and events are enabled. For
//! details on filtering spans and events using [`Subscribe`] implementations,
//! see the [`subscribe` module documentation].
//!
//! [`subscribe` module documentation]: crate::subscribe#filtering-with-subscribers
//! [`Subscribe`]: crate::subscribe
mod filter_fn;
>>>>>>> origin/master
mod level;

<<<<<<< HEAD
pub use self::filter_fn::*;
||||||| 386969ba
=======
feature! {
    #![all(feature = "env-filter", feature = "std")]
    mod env;
    pub use self::env::*;
}

feature! {
    #![all(feature = "registry", feature = "std")]
    mod subscriber_filters;
    pub use self::subscriber_filters::*;
}

pub use self::filter_fn::*;
#[cfg(not(feature = "registry"))]
pub(crate) use self::has_psf_stubs::*;

>>>>>>> origin/master
pub use self::level::{LevelFilter, ParseError as LevelParseError};

<<<<<<< HEAD
#[cfg(not(all(feature = "registry", feature = "std")))]
pub(crate) use self::has_plf_stubs::*;

feature! {
    #![any(feature = "std", feature = "alloc")]
    pub mod targets;
    pub use self::targets::Targets;

    mod directive;
    pub use self::directive::ParseError;
}

/// Stub implementations of the per-layer-fitler detection functions for when the
/// `registry` feature is disabled.
#[cfg(not(all(feature = "registry", feature = "std")))]
mod has_plf_stubs {
    pub(crate) fn is_plf_downcast_marker(_: core::any::TypeId) -> bool {
        false
    }

    /// Does a type implementing `Subscriber` contain any per-layer filters?
    pub(crate) fn subscriber_has_plf<S>(_: &S) -> bool
    where
        S: tracing_core::Subscriber,
    {
        false
    }

    /// Does a type implementing `Layer` contain any per-layer filters?
    pub(crate) fn layer_has_plf<L, S>(_: &L) -> bool
    where
        L: crate::Layer<S>,
        S: tracing_core::Subscriber,
    {
        false
    }
}
||||||| 386969ba
#[cfg(feature = "env-filter")]
#[cfg_attr(docsrs, doc(cfg(feature = "env-filter")))]
pub use self::env::*;
=======
#[cfg(not(all(feature = "registry", feature = "std")))]
#[allow(unused_imports)]
pub(crate) use self::has_psf_stubs::*;

feature! {
    #![any(feature = "std", feature = "alloc")]
    pub mod targets;
    pub use self::targets::Targets;

    mod directive;
    pub use self::directive::ParseError;
}

/// Stub implementations of the per-subscriber-filter detection functions for
/// when the `registry` feature is disabled.
#[cfg(not(all(feature = "registry", feature = "std")))]
mod has_psf_stubs {
    pub(crate) fn is_psf_downcast_marker(_: core::any::TypeId) -> bool {
        false
    }

    /// Does a type implementing `Collect` contain any per-subscriber filters?
    pub(crate) fn collector_has_psf<C>(_: &C) -> bool
    where
        C: tracing_core::Collect,
    {
        false
    }

    /// Does a type implementing `Subscribe` contain any per-subscriber filters?
    pub(crate) fn subscriber_has_psf<S, C>(_: &S) -> bool
    where
        S: crate::Subscribe<C>,
        C: tracing_core::Collect,
    {
        false
    }
}
>>>>>>> origin/master
