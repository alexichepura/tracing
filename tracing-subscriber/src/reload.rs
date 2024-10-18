//! Wrapper for a `Collect` or `Subscribe` to allow it to be dynamically reloaded.
//!
<<<<<<< HEAD
//! This module provides a [`Layer` type] implementing the [`Layer` trait] or [`Filter` trait]
//! which wraps another type implementing the corresponding trait. This
//! allows the wrapped type to be replaced with another
||||||| 386969ba
//! This module provides a [`Layer` type] which wraps another type implementing
//! the [`Layer` trait], allowing the wrapped type to be replaced with another
=======
//! This module provides a type implementing [`Subscribe`] and [`Filter`]
//! which wraps another type implementing the corresponding trait. This
//! allows the wrapped type to be replaced with another
>>>>>>> origin/master
//! instance of that type at runtime.
//!
<<<<<<< HEAD
//! This can be used in cases where a subset of `Layer` or `Filter` functionality
||||||| 386969ba
//! This can be used in cases where a subset of `Subscriber` functionality
=======
//! This can be used in cases where a subset of `Collect` or `Filter` functionality
>>>>>>> origin/master
//! should be dynamically reconfigured, such as when filtering directives may
//! change at runtime. Note that this subscriber introduces a (relatively small)
//! amount of overhead, and should thus only be used as needed.
//!
<<<<<<< HEAD
//! # Examples
//!
//! Reloading a [global filtering](crate::layer#global-filtering) layer:
//!
//! ```rust
//! # use tracing::info;
//! use tracing_subscriber::{filter, fmt, reload, prelude::*};
//! let filter = filter::LevelFilter::WARN;
//! let (filter, reload_handle) = reload::Layer::new(filter);
//! tracing_subscriber::registry()
//!   .with(filter)
//!   .with(fmt::Layer::default())
//!   .init();
//! #
//! # // specifying the Registry type is required
//! # let _: &reload::Handle<filter::LevelFilter, tracing_subscriber::Registry> = &reload_handle;
//! #
//! info!("This will be ignored");
//! reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO);
//! info!("This will be logged");
//! ```
//!
//! Reloading a [`Filtered`](crate::filter::Filtered) layer:
//!
//! ```rust
//! # use tracing::info;
//! use tracing_subscriber::{filter, fmt, reload, prelude::*};
//! let filtered_layer = fmt::Layer::default().with_filter(filter::LevelFilter::WARN);
//! let (filtered_layer, reload_handle) = reload::Layer::new(filtered_layer);
//! #
//! # // specifying the Registry type is required
//! # let _: &reload::Handle<filter::Filtered<fmt::Layer<tracing_subscriber::Registry>,
//! # filter::LevelFilter, tracing_subscriber::Registry>,tracing_subscriber::Registry>
//! # = &reload_handle;
//! #
//! tracing_subscriber::registry()
//!   .with(filtered_layer)
//!   .init();
//! info!("This will be ignored");
//! reload_handle.modify(|layer| *layer.filter_mut() = filter::LevelFilter::INFO);
//! info!("This will be logged");
//! ```
//!
//! ## Note
//!
//! The [`Layer`] implementation is unable to implement downcasting functionality,
//! so certain [`Layer`] will fail to downcast if wrapped in a `reload::Layer`.
//!
//! If you only want to be able to dynamically change the
//! `Filter` on a layer, prefer wrapping that `Filter` in the `reload::Layer`.
//!
//! [`Filter` trait]: crate::layer::Filter
//! [`Layer` type]: Layer
//! [`Layer` trait]: super::layer::Layer
use crate::layer;
||||||| 386969ba
//! [`Layer` type]: struct.Layer.html
//! [`Layer` trait]: ../layer/trait.Layer.html
use crate::layer;
=======
//! ## Note
//!
//! //! The [`Subscribe`] implementation is unable to implement downcasting functionality,
//! so certain `Subscribers` will fail to reload if wrapped in a `reload::Subscriber`.
//!
//! If you only want to be able to dynamically change the
//! `Filter` on your layer, prefer wrapping that `Filter` in the `reload::Subscriber`.
//!
//! [`Subscribe`]: crate::Subscribe
//! [`Filter`]: crate::subscribe::Filter
use crate::subscribe;
>>>>>>> origin/master
use crate::sync::RwLock;

<<<<<<< HEAD
use core::any::TypeId;
||||||| 386969ba
=======
use core::{any::TypeId, ptr::NonNull};
>>>>>>> origin/master
use std::{
    error, fmt,
    sync::{Arc, Weak},
};
use tracing_core::{
<<<<<<< HEAD
    callsite, span,
    subscriber::{Interest, Subscriber},
    Dispatch, Event, LevelFilter, Metadata,
||||||| 386969ba
    callsite, span,
    subscriber::{Interest, Subscriber},
    Event, Metadata,
=======
    callsite,
    collect::{Collect, Interest},
    span, Dispatch, Event, LevelFilter, Metadata,
>>>>>>> origin/master
};

<<<<<<< HEAD
/// Wraps a `Layer` or `Filter`, allowing it to be reloaded dynamically at runtime.
||||||| 386969ba
/// Wraps a `Layer`, allowing it to be reloaded dynamically at runtime.
=======
/// Wraps a `Filter` or `Subscribe`, allowing it to be reloaded dynamically at runtime.
///
/// [`Filter`]: crate::subscribe::Filter
/// [`Subscribe`]: crate::Subscribe
>>>>>>> origin/master
#[derive(Debug)]
pub struct Subscriber<S> {
    // TODO(eliza): this once used a `crossbeam_util::ShardedRwLock`. We may
    // eventually wish to replace it with a sharded lock implementation on top
    // of our internal `RwLock` wrapper type. If possible, we should profile
    // this first to determine if it's necessary.
    inner: Arc<RwLock<S>>,
}

<<<<<<< HEAD
/// Allows reloading the state of an associated [`Layer`](crate::layer::Layer).
||||||| 386969ba
/// Allows reloading the state of an associated `Layer`.
=======
/// Allows reloading the state of an associated `Collect`.
>>>>>>> origin/master
#[derive(Debug)]
pub struct Handle<S> {
    inner: Weak<RwLock<S>>,
}

/// Indicates that an error occurred when reloading a subscriber.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    CollectorGone,
    Poisoned,
}

// ===== impl Collect =====

impl<S, C> crate::Subscribe<C> for Subscriber<S>
where
    S: crate::Subscribe<C> + 'static,
    C: Collect,
{
<<<<<<< HEAD
    fn on_register_dispatch(&self, subscriber: &Dispatch) {
        try_lock!(self.inner.read()).on_register_dispatch(subscriber);
    }

    fn on_layer(&mut self, subscriber: &mut S) {
        try_lock!(self.inner.write(), else return).on_layer(subscriber);
    }

||||||| 386969ba
=======
    fn on_register_dispatch(&self, collector: &Dispatch) {
        try_lock!(self.inner.read()).on_register_dispatch(collector);
    }

>>>>>>> origin/master
    #[inline]
    fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest {
        try_lock!(self.inner.read(), else return Interest::sometimes()).register_callsite(metadata)
    }

    #[inline]
    fn enabled(&self, metadata: &Metadata<'_>, ctx: subscribe::Context<'_, C>) -> bool {
        try_lock!(self.inner.read(), else return false).enabled(metadata, ctx)
    }

    #[inline]
<<<<<<< HEAD
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: layer::Context<'_, S>) {
        try_lock!(self.inner.read()).on_new_span(attrs, id, ctx)
||||||| 386969ba
    fn new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: layer::Context<'_, S>) {
        try_lock!(self.inner.read()).new_span(attrs, id, ctx)
=======
    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        id: &span::Id,
        ctx: subscribe::Context<'_, C>,
    ) {
        try_lock!(self.inner.read()).on_new_span(attrs, id, ctx)
>>>>>>> origin/master
    }

    #[inline]
    fn on_record(
        &self,
        span: &span::Id,
        values: &span::Record<'_>,
        ctx: subscribe::Context<'_, C>,
    ) {
        try_lock!(self.inner.read()).on_record(span, values, ctx)
    }

    #[inline]
    fn on_follows_from(&self, span: &span::Id, follows: &span::Id, ctx: subscribe::Context<'_, C>) {
        try_lock!(self.inner.read()).on_follows_from(span, follows, ctx)
    }

    #[inline]
<<<<<<< HEAD
    fn event_enabled(&self, event: &Event<'_>, ctx: layer::Context<'_, S>) -> bool {
        try_lock!(self.inner.read(), else return false).event_enabled(event, ctx)
    }

    #[inline]
    fn on_event(&self, event: &Event<'_>, ctx: layer::Context<'_, S>) {
||||||| 386969ba
    fn on_event(&self, event: &Event<'_>, ctx: layer::Context<'_, S>) {
=======
    fn event_enabled(&self, event: &Event<'_>, ctx: subscribe::Context<'_, C>) -> bool {
        try_lock!(self.inner.read(), else return false).event_enabled(event, ctx)
    }

    #[inline]
    fn on_event(&self, event: &Event<'_>, ctx: subscribe::Context<'_, C>) {
>>>>>>> origin/master
        try_lock!(self.inner.read()).on_event(event, ctx)
    }

    #[inline]
    fn on_enter(&self, id: &span::Id, ctx: subscribe::Context<'_, C>) {
        try_lock!(self.inner.read()).on_enter(id, ctx)
    }

    #[inline]
    fn on_exit(&self, id: &span::Id, ctx: subscribe::Context<'_, C>) {
        try_lock!(self.inner.read()).on_exit(id, ctx)
    }

    #[inline]
    fn on_close(&self, id: span::Id, ctx: subscribe::Context<'_, C>) {
        try_lock!(self.inner.read()).on_close(id, ctx)
    }

    #[inline]
    fn on_id_change(&self, old: &span::Id, new: &span::Id, ctx: subscribe::Context<'_, C>) {
        try_lock!(self.inner.read()).on_id_change(old, new, ctx)
    }
<<<<<<< HEAD

    #[inline]
    fn max_level_hint(&self) -> Option<LevelFilter> {
        try_lock!(self.inner.read(), else return None).max_level_hint()
    }

    #[doc(hidden)]
    unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()> {
        // Safety: it is generally unsafe to downcast through a reload, because
        // the pointer can be invalidated after the lock is dropped.
        // `NoneLayerMarker` is a special case because it
        // is never dereferenced.
        //
        // Additionally, even if the marker type *is* dereferenced (which it
        // never will be), the pointer should be valid even if the subscriber
        // is reloaded, because all `NoneLayerMarker` pointers that we return
        // actually point to the global static singleton `NoneLayerMarker`,
        // rather than to a field inside the lock.
        if id == TypeId::of::<layer::NoneLayerMarker>() {
            return try_lock!(self.inner.read(), else return None).downcast_raw(id);
        }

        None
    }
||||||| 386969ba
=======

    #[inline]
    fn max_level_hint(&self) -> Option<LevelFilter> {
        try_lock!(self.inner.read(), else return None).max_level_hint()
    }

    #[doc(hidden)]
    unsafe fn downcast_raw(&self, id: TypeId) -> Option<NonNull<()>> {
        // Safety: it is generally unsafe to downcast through a reload, because
        // the pointer can be invalidated after the lock is dropped.
        // `NoneLayerMarker` is a special case because it
        // is never dereferenced.
        //
        // Additionally, even if the marker type *is* dereferenced (which it
        // never will be), the pointer should be valid even if the subscriber
        // is reloaded, because all `NoneLayerMarker` pointers that we return
        // actually point to the global static singleton `NoneLayerMarker`,
        // rather than to a field inside the lock.
        if id == TypeId::of::<subscribe::NoneLayerMarker>() {
            return try_lock!(self.inner.read(), else return None).downcast_raw(id);
        }

        None
    }
>>>>>>> origin/master
}

<<<<<<< HEAD
// ===== impl Filter =====

#[cfg(all(feature = "registry", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "registry", feature = "std"))))]
impl<S, L> crate::layer::Filter<S> for Layer<L, S>
||||||| 386969ba
impl<L, S> Layer<L, S>
=======
#[cfg(all(feature = "registry", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "registry", feature = "std"))))]
impl<S, C> crate::subscribe::Filter<C> for Subscriber<S>
>>>>>>> origin/master
where
<<<<<<< HEAD
    L: crate::layer::Filter<S> + 'static,
    S: Subscriber,
||||||| 386969ba
    L: crate::Layer<S> + 'static,
    S: Subscriber,
=======
    S: crate::subscribe::Filter<C> + 'static,
    C: Collect,
>>>>>>> origin/master
{
<<<<<<< HEAD
    #[inline]
    fn callsite_enabled(&self, metadata: &'static Metadata<'static>) -> Interest {
        try_lock!(self.inner.read(), else return Interest::sometimes()).callsite_enabled(metadata)
    }

    #[inline]
    fn enabled(&self, metadata: &Metadata<'_>, ctx: &layer::Context<'_, S>) -> bool {
        try_lock!(self.inner.read(), else return false).enabled(metadata, ctx)
    }

    #[inline]
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: layer::Context<'_, S>) {
        try_lock!(self.inner.read()).on_new_span(attrs, id, ctx)
    }

    #[inline]
    fn on_record(&self, span: &span::Id, values: &span::Record<'_>, ctx: layer::Context<'_, S>) {
        try_lock!(self.inner.read()).on_record(span, values, ctx)
    }

    #[inline]
    fn on_enter(&self, id: &span::Id, ctx: layer::Context<'_, S>) {
        try_lock!(self.inner.read()).on_enter(id, ctx)
    }

    #[inline]
    fn on_exit(&self, id: &span::Id, ctx: layer::Context<'_, S>) {
        try_lock!(self.inner.read()).on_exit(id, ctx)
    }

    #[inline]
    fn on_close(&self, id: span::Id, ctx: layer::Context<'_, S>) {
        try_lock!(self.inner.read()).on_close(id, ctx)
    }

    #[inline]
    fn max_level_hint(&self) -> Option<LevelFilter> {
        try_lock!(self.inner.read(), else return None).max_level_hint()
    }
}

impl<L, S> Layer<L, S> {
    /// Wraps the given [`Layer`] or [`Filter`], returning a `reload::Layer`
    /// and a `Handle` that allows the inner value to be modified at runtime.
    ///
    /// [`Layer`]: crate::layer::Layer
    /// [`Filter`]: crate::layer::Filter
    pub fn new(inner: L) -> (Self, Handle<L, S>) {
||||||| 386969ba
    /// Wraps the given `Layer`, returning a `Layer` and a `Handle` that allows
    /// the inner type to be modified at runtime.
    pub fn new(inner: L) -> (Self, Handle<L, S>) {
=======
    #[inline]
    fn callsite_enabled(&self, metadata: &'static Metadata<'static>) -> Interest {
        try_lock!(self.inner.read(), else return Interest::sometimes()).callsite_enabled(metadata)
    }

    #[inline]
    fn enabled(&self, metadata: &Metadata<'_>, ctx: &subscribe::Context<'_, C>) -> bool {
        try_lock!(self.inner.read(), else return false).enabled(metadata, ctx)
    }

    #[inline]
    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        id: &span::Id,
        ctx: subscribe::Context<'_, C>,
    ) {
        try_lock!(self.inner.read()).on_new_span(attrs, id, ctx)
    }

    #[inline]
    fn on_record(
        &self,
        span: &span::Id,
        values: &span::Record<'_>,
        ctx: subscribe::Context<'_, C>,
    ) {
        try_lock!(self.inner.read()).on_record(span, values, ctx)
    }

    #[inline]
    fn on_enter(&self, id: &span::Id, ctx: subscribe::Context<'_, C>) {
        try_lock!(self.inner.read()).on_enter(id, ctx)
    }

    #[inline]
    fn on_exit(&self, id: &span::Id, ctx: subscribe::Context<'_, C>) {
        try_lock!(self.inner.read()).on_exit(id, ctx)
    }

    #[inline]
    fn on_close(&self, id: span::Id, ctx: subscribe::Context<'_, C>) {
        try_lock!(self.inner.read()).on_close(id, ctx)
    }

    #[inline]
    fn max_level_hint(&self) -> Option<LevelFilter> {
        try_lock!(self.inner.read(), else return None).max_level_hint()
    }
}

impl<T> Subscriber<T> {
    /// Wraps the given `Subscribe` or `Filter`,
    /// returning a subscriber or filter and a `Handle` that allows
    /// the inner type to be modified at runtime.
    ///
    /// [`Filter`]: crate::subscribe::Filter
    /// [`Subscribe`]: crate::Subscribe
    pub fn new(inner: T) -> (Self, Handle<T>) {
>>>>>>> origin/master
        let this = Self {
            inner: Arc::new(RwLock::new(inner)),
        };
        let handle = this.handle();
        (this, handle)
    }

<<<<<<< HEAD
    /// Returns a `Handle` that can be used to reload the wrapped [`Layer`] or [`Filter`].
    ///
    /// [`Layer`]: crate::layer::Layer
    /// [`Filter`]: crate::filter::Filter
    pub fn handle(&self) -> Handle<L, S> {
||||||| 386969ba
    /// Returns a `Handle` that can be used to reload the wrapped `Layer`.
    pub fn handle(&self) -> Handle<L, S> {
=======
    /// Returns a `Handle` that can be used to reload the wrapped `Subscribe`.
    pub fn handle(&self) -> Handle<T> {
>>>>>>> origin/master
        Handle {
            inner: Arc::downgrade(&self.inner),
        }
    }
}

// ===== impl Handle =====

<<<<<<< HEAD
impl<L, S> Handle<L, S> {
    /// Replace the current [`Layer`] or [`Filter`] with the provided `new_value`.
    ///
    /// [`Handle::reload`] cannot be used with the [`Filtered`] layer; use
    /// [`Handle::modify`] instead (see [this issue] for additional details).
    ///
    /// However, if the _only_ the [`Filter`]  needs to be modified, use
    /// `reload::Layer` to wrap the `Filter` directly.
    ///
    /// [`Layer`]: crate::layer::Layer
    /// [`Filter`]: crate::layer::Filter
    /// [`Filtered`]: crate::filter::Filtered
    ///
    /// [this issue]: https://github.com/tokio-rs/tracing/issues/1629
    pub fn reload(&self, new_value: impl Into<L>) -> Result<(), Error> {
        self.modify(|layer| {
            *layer = new_value.into();
||||||| 386969ba
impl<L, S> Handle<L, S>
where
    L: crate::Layer<S> + 'static,
    S: Subscriber,
{
    /// Replace the current layer with the provided `new_layer`.
    pub fn reload(&self, new_layer: impl Into<L>) -> Result<(), Error> {
        self.modify(|layer| {
            *layer = new_layer.into();
=======
impl<T> Handle<T> {
    /// Replace the current subscriber or filter with the provided `new_value`.
    ///
    /// [`Handle::reload`] cannot be used with the [`Filtered`](crate::filter::Filtered)
    /// subscriber; use [`Handle::modify`] instead (see [this issue] for additional details).
    ///
    /// However, if the _only_ the [`Filter`](crate::subscribe::Filter) needs to be modified,
    /// use `reload::Subscriber` to wrap the `Filter` directly.
    ///
    /// [this issue]: https://github.com/tokio-rs/tracing/issues/1629
    pub fn reload(&self, new_value: impl Into<T>) -> Result<(), Error> {
        self.modify(|object| {
            *object = new_value.into();
>>>>>>> origin/master
        })
    }

<<<<<<< HEAD
    /// Invokes a closure with a mutable reference to the current layer or filter,
||||||| 386969ba
    /// Invokes a closure with a mutable reference to the current layer,
=======
    /// Invokes a closure with a mutable reference to the current subscriber,
>>>>>>> origin/master
    /// allowing it to be modified in place.
    pub fn modify(&self, f: impl FnOnce(&mut T)) -> Result<(), Error> {
        let inner = self.inner.upgrade().ok_or(Error {
            kind: ErrorKind::CollectorGone,
        })?;

        let mut lock = try_lock!(inner.write(), else return Err(Error::poisoned()));
        f(&mut *lock);
        // Release the lock before rebuilding the interest cache, as that
        // function will lock the new subscriber.
        drop(lock);

        callsite::rebuild_interest_cache();

        // If the `log` crate compatibility feature is in use, set `log`'s max
        // level as well, in case the max `tracing` level changed. We do this
        // *after* rebuilding the interest cache, as that's when the `tracing`
        // max level filter is re-computed.
        #[cfg(feature = "tracing-log")]
        tracing_log::log::set_max_level(tracing_log::AsLog::as_log(
            &crate::filter::LevelFilter::current(),
        ));

        Ok(())
    }

<<<<<<< HEAD
    /// Returns a clone of the layer or filter's current value if it still exists.
    /// Otherwise, if the subscriber has been dropped, returns `None`.
    pub fn clone_current(&self) -> Option<L>
||||||| 386969ba
    /// Returns a clone of the layer's current value if it still exists.
    /// Otherwise, if the subscriber has been dropped, returns `None`.
    pub fn clone_current(&self) -> Option<L>
=======
    /// Returns a clone of the subscriber's current value if it still exists.
    /// Otherwise, if the collector has been dropped, returns `None`.
    pub fn clone_current(&self) -> Option<T>
>>>>>>> origin/master
    where
        T: Clone,
    {
        self.with_current(T::clone).ok()
    }

<<<<<<< HEAD
    /// Invokes a closure with a borrowed reference to the current layer or filter,
    /// returning the result (or an error if the subscriber no longer exists).
    pub fn with_current<T>(&self, f: impl FnOnce(&L) -> T) -> Result<T, Error> {
||||||| 386969ba
    /// Invokes a closure with a borrowed reference to the current layer,
    /// returning the result (or an error if the subscriber no longer exists).
    pub fn with_current<T>(&self, f: impl FnOnce(&L) -> T) -> Result<T, Error> {
=======
    /// Invokes a closure with a borrowed reference to the current subscriber,
    /// returning the result (or an error if the collector no longer exists).
    pub fn with_current<T2>(&self, f: impl FnOnce(&T) -> T2) -> Result<T2, Error> {
>>>>>>> origin/master
        let inner = self.inner.upgrade().ok_or(Error {
            kind: ErrorKind::CollectorGone,
        })?;
        let inner = try_lock!(inner.read(), else return Err(Error::poisoned()));
        Ok(f(&*inner))
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            inner: self.inner.clone(),
        }
    }
}

// ===== impl Error =====

impl Error {
    fn poisoned() -> Self {
        Self {
            kind: ErrorKind::Poisoned,
        }
    }

    /// Returns `true` if this error occurred because the subscriber was poisoned by
    /// a panic on another thread.
    pub fn is_poisoned(&self) -> bool {
        matches!(self.kind, ErrorKind::Poisoned)
    }

    /// Returns `true` if this error occurred because the `Collector`
    /// containing the reloadable subscriber was dropped.
    pub fn is_dropped(&self) -> bool {
<<<<<<< HEAD
        matches!(self.kind, ErrorKind::SubscriberGone)
||||||| 386969ba
        match self.kind {
            ErrorKind::SubscriberGone => true,
            _ => false,
        }
=======
        matches!(self.kind, ErrorKind::CollectorGone)
>>>>>>> origin/master
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self.kind {
            ErrorKind::CollectorGone => "subscriber no longer exists",
            ErrorKind::Poisoned => "lock poisoned",
        };
        f.pad(msg)
    }
}

impl error::Error for Error {}
