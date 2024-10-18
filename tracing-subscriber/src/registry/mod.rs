//! Storage for span data shared by multiple [`Subscribe`]s.
//!
//! ## Using the Span Registry
//!
//! This module provides the [`Registry`] type, a [`Collect`] implementation
//! which tracks per-span data and exposes it to subscribers. When a `Registry`
//! is used as the base `Collect` of a `Subscribe` stack, the
//! [`subscribe::Context`][ctx] type will provide methods allowing subscribers to
//! [look up span data][lookup] stored in the registry. While [`Registry`] is a
//! reasonable default for storing spans and events, other stores that implement
//! [`LookupSpan`] and [`Collect`] themselves (with [`SpanData`] implemented
//! by the per-span data they store) can be used as a drop-in replacement.
//!
//! For example, we might create a `Registry` and add multiple `Subscriber`s like so:
//! ```rust
//! use tracing_subscriber::{registry::Registry, Subscribe, prelude::*};
//! # use tracing_core::Collect;
//! # pub struct FooSubscriber {}
//! # pub struct BarSubscriber {}
//! # impl<C: Collect> Subscribe<C> for FooSubscriber {}
//! # impl<C: Collect> Subscribe<C> for BarSubscriber {}
//! # impl FooSubscriber {
//! # fn new() -> Self { Self {} }
//! # }
//! # impl BarSubscriber {
//! # fn new() -> Self { Self {} }
//! # }
//!
//! let subscriber = Registry::default()
//!     .with(FooSubscriber::new())
//!     .with(BarSubscriber::new());
//! ```
//!
//! If a type implementing `Subscribe` depends on the functionality of a `Registry`
//! implementation, it should bound its `Collect` type parameter with the
//! [`LookupSpan`] trait, like so:
//!
//! ```rust
//! use tracing_subscriber::{registry, Subscribe};
//! use tracing_core::Collect;
//!
//! pub struct MySubscriber {
//!     // ...
//! }
//!
//! impl<C> Subscribe<C> for MySubscriber
//! where
//!     C: Collect + for<'a> registry::LookupSpan<'a>,
//! {
//!     // ...
//! }
//! ```
//! When this bound is added, the subscriber implementation will be guaranteed
//! access to the [`Context`][ctx] methods, such as [`Context::span`][lookup], that
//! require the root collector to be a registry.
//!
<<<<<<< HEAD
//! [`Layer`]: crate::layer::Layer
//! [`Subscriber`]: tracing_core::Subscriber
//! [ctx]: crate::layer::Context
//! [lookup]: crate::layer::Context::span()
||||||| 386969ba
//! [`Layer`]: ../layer/trait.Layer.html
//! [`Subscriber`]:
//!     https://docs.rs/tracing-core/latest/tracing_core/subscriber/trait.Subscriber.html
//! [`Registry`]: struct.Registry.html
//! [ctx]: ../layer/struct.Context.html
//! [lookup]: ../layer/struct.Context.html#method.span
//! [`LookupSpan`]: trait.LookupSpan.html
//! [`SpanData`]: trait.SpanData.html
=======
//! [`Subscribe`]: crate::subscribe::Subscribe
//! [`Collect`]: tracing_core::collect::Collect
//! [ctx]: crate::subscribe::Context
//! [lookup]: crate::subscribe::Context::span()
use core::fmt::Debug;

>>>>>>> origin/master
use tracing_core::{field::FieldSet, span::Id, Metadata};

feature! {
    #![feature = "std"]
    /// A module containing a type map of span extensions.
    mod extensions;
    pub use extensions::{Extensions, ExtensionsMut};

}

feature! {
    #![all(feature = "registry", feature = "std")]

    mod sharded;
    mod stack;

    pub use sharded::Data;
    pub use sharded::Registry;

    use crate::filter::FilterId;
}

/// Provides access to stored span data.
///
/// Subscribers which store span data and associate it with span IDs should
/// implement this trait; if they do, any [`Subscriber`]s wrapping them can look up
/// metadata via the [`Context`] type's [`span()`] method.
///
<<<<<<< HEAD
/// [`Layer`]: super::layer::Layer
/// [`Context`]: super::layer::Context
/// [`span()`]: super::layer::Context::span
||||||| 386969ba
/// [`Layer`]: ../layer/trait.Layer.html
/// [`Context`]: ../layer/struct.Context.html
/// [`span()`]: ../layer/struct.Context.html#method.span
=======
/// [`Subscriber`]: crate::Subscribe
/// [`Context`]: crate::subscribe::Context
/// [`span()`]: crate::subscribe::Context::span()
>>>>>>> origin/master
pub trait LookupSpan<'a> {
    /// The type of span data stored in this registry.
    type Data: SpanData<'a>;

    /// Returns the [`SpanData`] for a given [`Id`], if it exists.
    ///
<<<<<<< HEAD
||||||| 386969ba
    /// <div class="information">
    ///     <div class="tooltip ignore" style="">ⓘ<span class="tooltiptext">Note</span></div>
    /// </div>
    /// <div class="example-wrap" style="display:inline-block">
=======
    /// <div class="example-wrap" style="display:inline-block">
>>>>>>> origin/master
    /// <pre class="ignore" style="white-space:normal;font:inherit;">
<<<<<<< HEAD
    /// <strong>Note</strong>: users of the <code>LookupSpan</code> trait should
    /// typically call the <a href="#method.span"><code>span</code></a> method rather
    /// than this method. The <code>span</code> method is implemented by
    /// <em>calling</em> <code>span_data</code>, but returns a reference which is
    /// capable of performing more sophisiticated queries.
    /// </pre>
||||||| 386969ba
    /// <strong>Note</strong>: users of the <code>LookupSpan</code> trait should
    /// typically call the <a href="#method.span"><code>span</code></a> method rather
    /// than this method. The <code>span</code> method is implemented by
    /// <em>calling</em> <code>span_data</code>, but returns a reference which is
    /// capable of performing more sophisiticated queries.
    /// </pre></div>
=======
    ///
    /// **Note**: users of the `LookupSpan` trait should
    /// typically call the [`span`][Self::span] method rather
    /// than this method. The `span` method is implemented by
    /// *calling* `span_data`, but returns a reference which is
    /// capable of performing more sophisticated queries.
    ///
    /// </pre></div>
>>>>>>> origin/master
    ///
    fn span_data(&'a self, id: &Id) -> Option<Self::Data>;

    /// Returns a [`SpanRef`] for the span with the given `Id`, if it exists.
    ///
    /// A `SpanRef` is similar to [`SpanData`], but it allows performing
    /// additional lookups against the registry that stores the wrapped data.
    ///
    /// In general, _users_ of the `LookupSpan` trait should use this method
    /// rather than the [`span_data`] method; while _implementors_ of this trait
    /// should only implement `span_data`.
    ///
    /// [`span_data`]: LookupSpan::span_data()
    fn span(&'a self, id: &Id) -> Option<SpanRef<'_, Self>>
    where
        Self: Sized,
    {
        let data = self.span_data(id)?;
        Some(SpanRef {
            registry: self,
            data,
            #[cfg(feature = "registry")]
            filter: FilterId::none(),
        })
    }
<<<<<<< HEAD

    /// Registers a [`Filter`] for [per-layer filtering] with this
    /// [`Subscriber`].
    ///
    /// The [`Filter`] can then use the returned [`FilterId`] to
    /// [check if it previously enabled a span][check].
    ///
    /// # Panics
    ///
    /// If this `Subscriber` does not support [per-layer filtering].
    ///
    /// [`Filter`]: crate::layer::Filter
    /// [per-layer filtering]: crate::layer::Layer#per-layer-filtering
    /// [`Subscriber`]: tracing_core::Subscriber
    /// [`FilterId`]: crate::filter::FilterId
    /// [check]: SpanData::is_enabled_for
    #[cfg(feature = "registry")]
    #[cfg_attr(docsrs, doc(cfg(feature = "registry")))]
    fn register_filter(&mut self) -> FilterId {
        panic!(
            "{} does not currently support filters",
            std::any::type_name::<Self>()
        )
    }
||||||| 386969ba
=======

    /// Registers a [`Filter`] for [per-subscriber filtering] with this
    /// [collector].
    ///
    /// The [`Filter`] can then use the returned [`FilterId`] to
    /// [check if it previously enabled a span][check].
    ///
    /// # Panics
    ///
    /// If this collector does not support [per-subscriber filtering].
    ///
    /// [`Filter`]: crate::subscribe::Filter
    /// [per-subscriber filtering]: crate::subscribe#per-subscriber-filtering
    /// [collector]: tracing_core::Collect
    /// [`FilterId`]: crate::filter::FilterId
    /// [check]: SpanData::is_enabled_for
    #[cfg(feature = "registry")]
    #[cfg_attr(docsrs, doc(cfg(feature = "registry")))]
    fn register_filter(&mut self) -> FilterId {
        panic!(
            "{} does not currently support filters",
            std::any::type_name::<Self>()
        )
    }
>>>>>>> origin/master
}

/// A stored representation of data associated with a span.
pub trait SpanData<'a> {
    /// Returns this span's ID.
    fn id(&self) -> Id;

    /// Returns a reference to the span's `Metadata`.
    fn metadata(&self) -> &'static Metadata<'static>;

    /// Returns a reference to the ID
    fn parent(&self) -> Option<&Id>;

    /// Returns a reference to this span's `Extensions`.
    ///
    /// The extensions may be used by `Subscriber`s to store additional data
    /// describing the span.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn extensions(&self) -> Extensions<'_>;

    /// Returns a mutable reference to this span's `Extensions`.
    ///
    /// The extensions may be used by `Subscriber`s to store additional data
    /// describing the span.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn extensions_mut(&self) -> ExtensionsMut<'_>;
<<<<<<< HEAD

    /// Returns `true` if this span is enabled for the [per-layer filter][plf]
    /// corresponding to the provided [`FilterId`].
    ///
    /// ## Default Implementation
    ///
    /// By default, this method assumes that the [`LookupSpan`] implementation
    /// does not support [per-layer filtering][plf], and always returns `true`.
    ///
    /// [plf]: crate::layer::Layer#per-layer-filtering
    /// [`FilterId`]: crate::filter::FilterId
    #[cfg(feature = "registry")]
    #[cfg_attr(docsrs, doc(cfg(feature = "registry")))]
    fn is_enabled_for(&self, filter: FilterId) -> bool {
        let _ = filter;
        true
    }
||||||| 386969ba
=======

    /// Returns `true` if this span is enabled for the [per-subscriber filter][psf]
    /// corresponding to the provided [`FilterId`].
    ///
    /// ## Default Implementation
    ///
    /// By default, this method assumes that the [`LookupSpan`] implementation
    /// does not support [per-subscriber filtering][psf], and always returns `true`.
    ///
    /// [psf]: crate::subscribe#per-subscriber-filtering
    /// [`FilterId`]: crate::filter::FilterId
    #[cfg(feature = "registry")]
    #[cfg_attr(docsrs, doc(cfg(feature = "registry")))]
    fn is_enabled_for(&self, filter: FilterId) -> bool {
        let _ = filter;
        true
    }
>>>>>>> origin/master
}

/// A reference to [span data] and the associated [registry].
///
/// This type implements all the same methods as [`SpanData`], and provides
/// additional methods for querying the registry based on values from the span.
///
/// [registry]: LookupSpan
#[derive(Debug)]
pub struct SpanRef<'a, R: LookupSpan<'a>> {
    registry: &'a R,
    data: R::Data,

    #[cfg(feature = "registry")]
    filter: FilterId,
}

/// An iterator over the parents of a span, ordered from leaf to root.
///
/// This is returned by the [`SpanRef::scope`] method.
#[derive(Debug)]
pub struct Scope<'a, R> {
    registry: &'a R,
    next: Option<Id>,

    #[cfg(all(feature = "registry", feature = "std"))]
    filter: FilterId,
}

<<<<<<< HEAD
feature! {
    #![any(feature = "alloc", feature = "std")]

||||||| 386969ba
/// An iterator over a span's parents, starting with the root of the trace
/// tree.
///
/// For additonal details, see [`SpanRef::from_root`].
///
/// [`Span::from_root`]: struct.SpanRef.html#method.from_root
pub struct FromRoot<'a, R: LookupSpan<'a>> {
    #[cfg(feature = "smallvec")]
    inner: std::iter::Rev<smallvec::IntoIter<SpanRefVecArray<'a, R>>>,
=======
feature! {
    #![any(feature = "alloc", feature = "std")]

    use alloc::{
        boxed::Box,
        sync::Arc
    };

>>>>>>> origin/master
    #[cfg(not(feature = "smallvec"))]
<<<<<<< HEAD
    use alloc::vec::{self, Vec};

    use core::{fmt,iter};

    /// An iterator over the parents of a span, ordered from root to leaf.
    ///
    /// This is returned by the [`Scope::from_root`] method.
    pub struct ScopeFromRoot<'a, R>
    where
        R: LookupSpan<'a>,
    {
        #[cfg(feature = "smallvec")]
        spans: iter::Rev<smallvec::IntoIter<SpanRefVecArray<'a, R>>>,
        #[cfg(not(feature = "smallvec"))]
        spans: iter::Rev<vec::IntoIter<SpanRef<'a, R>>>,
    }

    #[cfg(feature = "smallvec")]
    type SpanRefVecArray<'span, L> = [SpanRef<'span, L>; 16];

    impl<'a, R> Scope<'a, R>
    where
        R: LookupSpan<'a>,
    {
        /// Flips the order of the iterator, so that it is ordered from root to leaf.
        ///
        /// The iterator will first return the root span, then that span's immediate child,
        /// and so on until it finally returns the span that [`SpanRef::scope`] was called on.
        ///
        /// If any items were consumed from the [`Scope`] before calling this method then they
        /// will *not* be returned from the [`ScopeFromRoot`].
        ///
        /// **Note**: this will allocate if there are many spans remaining, or if the
        /// "smallvec" feature flag is not enabled.
        #[allow(clippy::wrong_self_convention)]
        pub fn from_root(self) -> ScopeFromRoot<'a, R> {
            #[cfg(feature = "smallvec")]
            type Buf<T> = smallvec::SmallVec<T>;
            #[cfg(not(feature = "smallvec"))]
            type Buf<T> = Vec<T>;
            ScopeFromRoot {
                spans: self.collect::<Buf<_>>().into_iter().rev(),
            }
        }
    }

    impl<'a, R> Iterator for ScopeFromRoot<'a, R>
    where
        R: LookupSpan<'a>,
    {
        type Item = SpanRef<'a, R>;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.spans.next()
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.spans.size_hint()
        }
    }

    impl<'a, R> fmt::Debug for ScopeFromRoot<'a, R>
    where
        R: LookupSpan<'a>,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.pad("ScopeFromRoot { .. }")
        }
    }
||||||| 386969ba
    inner: std::iter::Rev<std::vec::IntoIter<SpanRef<'a, R>>>,
=======
    use alloc::vec::{self, Vec};
    use core::{fmt,iter};

    /// An iterator over the parents of a span, ordered from root to leaf.
    ///
    /// This is returned by the [`Scope::from_root`] method.
    pub struct ScopeFromRoot<'a, R>
    where
        R: LookupSpan<'a>,
    {
        #[cfg(feature = "smallvec")]
        spans: iter::Rev<smallvec::IntoIter<SpanRefVecArray<'a, R>>>,
        #[cfg(not(feature = "smallvec"))]
        spans: iter::Rev<vec::IntoIter<SpanRef<'a, R>>>,
    }

    #[cfg(feature = "smallvec")]
    type SpanRefVecArray<'span, L> = [SpanRef<'span, L>; 16];

    impl<'a, S> LookupSpan<'a> for Arc<S>
    where
        S: LookupSpan<'a>,
    {
        type Data = <S as LookupSpan<'a>>::Data;

        fn span_data(&'a self, id: &Id) -> Option<Self::Data> {
            self.as_ref().span_data(id)
        }

        fn span(&'a self, id: &Id) -> Option<SpanRef<'_, Self>>
        where
            Self: Sized,
        {
            self.as_ref().span(id).map(
                |SpanRef {
                    registry: _,
                    data,
                    #[cfg(feature = "registry")]
                    filter,
                }| SpanRef {
                    registry: self,
                    data,
                    #[cfg(feature = "registry")]
                    filter,
                },
            )
        }
    }

    impl<'a, S> LookupSpan<'a> for Box<S>
    where
        S: LookupSpan<'a>,
    {
        type Data = <S as LookupSpan<'a>>::Data;

        fn span_data(&'a self, id: &Id) -> Option<Self::Data> {
            self.as_ref().span_data(id)
        }

        fn span(&'a self, id: &Id) -> Option<SpanRef<'_, Self>>
        where
            Self: Sized,
        {
            self.as_ref().span(id).map(
                |SpanRef {
                    registry: _,
                    data,
                    #[cfg(feature = "registry")]
                    filter,
                }| SpanRef {
                    registry: self,
                    data,
                    #[cfg(feature = "registry")]
                    filter,
                },
            )
        }
    }

    impl<'a, R> Scope<'a, R>
    where
        R: LookupSpan<'a>,
    {
        /// Flips the order of the iterator, so that it is ordered from root to leaf.
        ///
        /// The iterator will first return the root span, then that span's immediate child,
        /// and so on until it finally returns the span that [`SpanRef::scope`] was called on.
        ///
        /// If any items were consumed from the [`Scope`] before calling this method then they
        /// will *not* be returned from the [`ScopeFromRoot`].
        ///
        /// **Note**: this will allocate if there are many spans remaining, or if the
        /// "smallvec" feature flag is not enabled.
        #[allow(clippy::wrong_self_convention)]
        pub fn from_root(self) -> ScopeFromRoot<'a, R> {
            #[cfg(feature = "smallvec")]
            type Buf<T> = smallvec::SmallVec<T>;
            #[cfg(not(feature = "smallvec"))]
            type Buf<T> = Vec<T>;
            ScopeFromRoot {
                spans: self.collect::<Buf<_>>().into_iter().rev(),
            }
        }
    }

    impl<'a, R> Iterator for ScopeFromRoot<'a, R>
    where
        R: LookupSpan<'a>,
    {
        type Item = SpanRef<'a, R>;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.spans.next()
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.spans.size_hint()
        }
    }

    impl<'a, R> fmt::Debug for ScopeFromRoot<'a, R>
    where
        R: LookupSpan<'a>,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.pad("ScopeFromRoot { .. }")
        }
    }
>>>>>>> origin/master
}

impl<'a, R> Iterator for Scope<'a, R>
where
    R: LookupSpan<'a>,
{
    type Item = SpanRef<'a, R>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let curr = self.registry.span(self.next.as_ref()?)?;

            #[cfg(all(feature = "registry", feature = "std"))]
            let curr = curr.with_filter(self.filter);
            self.next = curr.data.parent().cloned();

            // If the `Scope` is filtered, check if the current span is enabled
            // by the selected filter ID.

            #[cfg(all(feature = "registry", feature = "std"))]
            {
                if !curr.is_enabled_for(self.filter) {
                    // The current span in the chain is disabled for this
                    // filter. Try its parent.
                    continue;
                }
            }

            return Some(curr);
        }
    }
}

impl<'a, R> SpanRef<'a, R>
where
    R: LookupSpan<'a>,
{
    /// Returns this span's ID.
    pub fn id(&self) -> Id {
        self.data.id()
    }

    /// Returns a static reference to the span's metadata.
    pub fn metadata(&self) -> &'static Metadata<'static> {
        self.data.metadata()
    }

    /// Returns the span's name,
    pub fn name(&self) -> &'static str {
        self.data.metadata().name()
    }

    /// Returns a list of [fields] defined by the span.
    ///
    /// [fields]: tracing_core::field
    pub fn fields(&self) -> &FieldSet {
        self.data.metadata().fields()
    }

<<<<<<< HEAD
||||||| 386969ba
    /// Returns the ID of this span's parent, or `None` if this span is the root
    /// of its trace tree.
    pub fn parent_id(&self) -> Option<&Id> {
        self.data.parent()
    }

=======
    /// Returns the ID of this span's parent, or `None` if this span is the root
    /// of its trace tree.
    #[deprecated(
        note = "this method cannot properly support per-subscriber filtering, and may \
            return the `Id` of a disabled span if per-subscriber filtering is in \
            use. use `.parent().map(SpanRef::id)` instead.",
        since = "0.2.21"
    )]
    pub fn parent_id(&self) -> Option<&Id> {
        // XXX(eliza): this doesn't work with PSF because the ID is potentially
        // borrowed from a parent we got from the registry, rather than from
        // `self`, so we can't return a borrowed parent. so, right now, we just
        // return the actual parent ID, and ignore PSF. which is not great.
        //
        // i think if we want this to play nice with PSF, we should just change
        // it to return the `Id` by value instead of `&Id` (which we ought to do
        // anyway since an `Id` is just a word) but that's a breaking change.
        // alternatively, we could deprecate this method since it can't support
        // PSF in its current form (which is what we would want to do if we want
        // to release PSF in a minor version)...

        // let mut id = self.data.parent()?;
        // loop {
        //     // Is this parent enabled by our filter?
        //     if self
        //         .filter
        //         .map(|filter| self.registry.is_enabled_for(id, filter))
        //         .unwrap_or(true)
        //     {
        //         return Some(id);
        //     }
        //     id = self.registry.span_data(id)?.parent()?;
        // }
        self.data.parent()
    }

>>>>>>> origin/master
    /// Returns a `SpanRef` describing this span's parent, or `None` if this
    /// span is the root of its trace tree.

    pub fn parent(&self) -> Option<Self> {
        let id = self.data.parent()?;
        let data = self.registry.span_data(id)?;

        #[cfg(all(feature = "registry", feature = "std"))]
        {
            // move these into mut bindings if the registry feature is enabled,
            // since they may be mutated in the loop.
            let mut data = data;
            loop {
                // Is this parent enabled by our filter?
                if data.is_enabled_for(self.filter) {
                    return Some(Self {
                        registry: self.registry,
                        filter: self.filter,
                        data,
                    });
                }

                // It's not enabled. If the disabled span has a parent, try that!
                let id = data.parent()?;
                data = self.registry.span_data(id)?;
            }
        }

        #[cfg(not(all(feature = "registry", feature = "std")))]
        Some(Self {
            registry: self.registry,
            data,
        })
    }

    /// Returns an iterator over all parents of this span, starting with this span,
    /// ordered from leaf to root.
    ///
<<<<<<< HEAD
    /// The iterator will first return the span, then the span's immediate parent,
    /// followed by that span's parent, and so on, until it reaches a root span.
    ///
    /// ```rust
    /// use tracing::{span, Subscriber};
    /// use tracing_subscriber::{
    ///     layer::{Context, Layer},
    ///     prelude::*,
    ///     registry::LookupSpan,
    /// };
    ///
    /// struct PrintingLayer;
    /// impl<S> Layer<S> for PrintingLayer
    /// where
    ///     S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    /// {
    ///     fn on_enter(&self, id: &span::Id, ctx: Context<S>) {
    ///         let span = ctx.span(id).unwrap();
    ///         let scope = span.scope().map(|span| span.name()).collect::<Vec<_>>();
    ///         println!("Entering span: {:?}", scope);
    ///     }
    /// }
    ///
    /// tracing::subscriber::with_default(tracing_subscriber::registry().with(PrintingLayer), || {
    ///     let _root = tracing::info_span!("root").entered();
    ///     // Prints: Entering span: ["root"]
    ///     let _child = tracing::info_span!("child").entered();
    ///     // Prints: Entering span: ["child", "root"]
    ///     let _leaf = tracing::info_span!("leaf").entered();
    ///     // Prints: Entering span: ["leaf", "child", "root"]
    /// });
    /// ```
    ///
    /// If the opposite order (from the root to this span) is desired, calling [`Scope::from_root`] on
    /// the returned iterator reverses the order.
    ///
    /// ```rust
    /// # use tracing::{span, Subscriber};
    /// # use tracing_subscriber::{
    /// #     layer::{Context, Layer},
    /// #     prelude::*,
    /// #     registry::LookupSpan,
    /// # };
    /// # struct PrintingLayer;
    /// impl<S> Layer<S> for PrintingLayer
    /// where
    ///     S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    /// {
    ///     fn on_enter(&self, id: &span::Id, ctx: Context<S>) {
    ///         let span = ctx.span(id).unwrap();
    ///         let scope = span.scope().from_root().map(|span| span.name()).collect::<Vec<_>>();
    ///         println!("Entering span: {:?}", scope);
    ///     }
    /// }
    ///
    /// tracing::subscriber::with_default(tracing_subscriber::registry().with(PrintingLayer), || {
    ///     let _root = tracing::info_span!("root").entered();
    ///     // Prints: Entering span: ["root"]
    ///     let _child = tracing::info_span!("child").entered();
    ///     // Prints: Entering span: ["root", "child"]
    ///     let _leaf = tracing::info_span!("leaf").entered();
    ///     // Prints: Entering span: ["root", "child", "leaf"]
    /// });
    /// ```
    pub fn scope(&self) -> Scope<'a, R> {
        Scope {
||||||| 386969ba
    /// The iterator will first return the span's immediate parent, followed by
    /// that span's parent, followed by _that_ span's parent, and so on, until a
    /// it reaches a root span.
    pub fn parents(&self) -> Parents<'a, R> {
        Parents {
=======
    /// The iterator will first return the span, then the span's immediate parent,
    /// followed by that span's parent, and so on, until it reaches a root span.
    ///
    /// ```rust
    /// use tracing::{span, Collect};
    /// use tracing_subscriber::{
    ///     subscribe::{Context, Subscribe},
    ///     prelude::*,
    ///     registry::LookupSpan,
    /// };
    ///
    /// struct PrintingSubscriber;
    /// impl<C> Subscribe<C> for PrintingSubscriber
    /// where
    ///     C: Collect + for<'lookup> LookupSpan<'lookup>,
    /// {
    ///     fn on_enter(&self, id: &span::Id, ctx: Context<C>) {
    ///         let span = ctx.span(id).unwrap();
    ///         let scope = span.scope().map(|span| span.name()).collect::<Vec<_>>();
    ///         println!("Entering span: {:?}", scope);
    ///     }
    /// }
    ///
    /// tracing::collect::with_default(tracing_subscriber::registry().with(PrintingSubscriber), || {
    ///     let _root = tracing::info_span!("root").entered();
    ///     // Prints: Entering span: ["root"]
    ///     let _child = tracing::info_span!("child").entered();
    ///     // Prints: Entering span: ["child", "root"]
    ///     let _leaf = tracing::info_span!("leaf").entered();
    ///     // Prints: Entering span: ["leaf", "child", "root"]
    /// });
    /// ```
    ///
    /// If the opposite order (from the root to this span) is desired, calling [`Scope::from_root`] on
    /// the returned iterator reverses the order.
    ///
    /// ```rust
    /// # use tracing::{span, Collect};
    /// # use tracing_subscriber::{
    /// #     subscribe::{Context, Subscribe},
    /// #     prelude::*,
    /// #     registry::LookupSpan,
    /// # };
    /// # struct PrintingSubscriber;
    /// impl<C> Subscribe<C> for PrintingSubscriber
    /// where
    ///     C: Collect + for<'lookup> LookupSpan<'lookup>,
    /// {
    ///     fn on_enter(&self, id: &span::Id, ctx: Context<C>) {
    ///         let span = ctx.span(id).unwrap();
    ///         let scope = span.scope().from_root().map(|span| span.name()).collect::<Vec<_>>();
    ///         println!("Entering span: {:?}", scope);
    ///     }
    /// }
    ///
    /// tracing::collect::with_default(tracing_subscriber::registry().with(PrintingSubscriber), || {
    ///     let _root = tracing::info_span!("root").entered();
    ///     // Prints: Entering span: ["root"]
    ///     let _child = tracing::info_span!("child").entered();
    ///     // Prints: Entering span: ["root", "child"]
    ///     let _leaf = tracing::info_span!("leaf").entered();
    ///     // Prints: Entering span: ["root", "child", "leaf"]
    /// });
    /// ```
    pub fn scope(&self) -> Scope<'a, R> {
        Scope {
>>>>>>> origin/master
            registry: self.registry,
            next: Some(self.id()),

            #[cfg(feature = "registry")]
            filter: self.filter,
        }
    }

    /// Returns a reference to this span's `Extensions`.
    ///
    /// The extensions may be used by `Subscriber`s to store additional data
    /// describing the span.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn extensions(&self) -> Extensions<'_> {
        self.data.extensions()
    }

    /// Returns a mutable reference to this span's `Extensions`.
    ///
    /// The extensions may be used by `Subscriber`s to store additional data
    /// describing the span.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn extensions_mut(&self) -> ExtensionsMut<'_> {
        self.data.extensions_mut()
    }

    #[cfg(all(feature = "registry", feature = "std"))]
    pub(crate) fn try_with_filter(self, filter: FilterId) -> Option<Self> {
        if self.is_enabled_for(filter) {
            return Some(self.with_filter(filter));
        }

        None
    }

    #[inline]
    #[cfg(all(feature = "registry", feature = "std"))]
    pub(crate) fn is_enabled_for(&self, filter: FilterId) -> bool {
        self.data.is_enabled_for(filter)
    }

    #[inline]
    #[cfg(all(feature = "registry", feature = "std"))]
    fn with_filter(self, filter: FilterId) -> Self {
        Self { filter, ..self }
    }
}

<<<<<<< HEAD
#[cfg(all(test, feature = "registry", feature = "std"))]
mod tests {
    use crate::{
        layer::{Context, Layer},
        prelude::*,
        registry::LookupSpan,
    };
    use std::sync::{Arc, Mutex};
    use tracing::{span, Subscriber};

    #[test]
    fn spanref_scope_iteration_order() {
        let last_entered_scope = Arc::new(Mutex::new(Vec::new()));

        #[derive(Default)]
        struct PrintingLayer {
            last_entered_scope: Arc<Mutex<Vec<&'static str>>>,
        }

        impl<S> Layer<S> for PrintingLayer
        where
            S: Subscriber + for<'lookup> LookupSpan<'lookup>,
        {
            fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
                let span = ctx.span(id).unwrap();
                let scope = span.scope().map(|span| span.name()).collect::<Vec<_>>();
                *self.last_entered_scope.lock().unwrap() = scope;
            }
        }

        let _guard = tracing::subscriber::set_default(crate::registry().with(PrintingLayer {
            last_entered_scope: last_entered_scope.clone(),
        }));

        let _root = tracing::info_span!("root").entered();
        assert_eq!(&*last_entered_scope.lock().unwrap(), &["root"]);
        let _child = tracing::info_span!("child").entered();
        assert_eq!(&*last_entered_scope.lock().unwrap(), &["child", "root"]);
        let _leaf = tracing::info_span!("leaf").entered();
        assert_eq!(
            &*last_entered_scope.lock().unwrap(),
            &["leaf", "child", "root"]
        );
    }

    #[test]
    fn spanref_scope_fromroot_iteration_order() {
        let last_entered_scope = Arc::new(Mutex::new(Vec::new()));

        #[derive(Default)]
        struct PrintingLayer {
            last_entered_scope: Arc<Mutex<Vec<&'static str>>>,
        }

        impl<S> Layer<S> for PrintingLayer
        where
            S: Subscriber + for<'lookup> LookupSpan<'lookup>,
        {
            fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
                let span = ctx.span(id).unwrap();
                let scope = span
                    .scope()
                    .from_root()
                    .map(|span| span.name())
                    .collect::<Vec<_>>();
                *self.last_entered_scope.lock().unwrap() = scope;
            }
        }

        let _guard = tracing::subscriber::set_default(crate::registry().with(PrintingLayer {
            last_entered_scope: last_entered_scope.clone(),
        }));

        let _root = tracing::info_span!("root").entered();
        assert_eq!(&*last_entered_scope.lock().unwrap(), &["root"]);
        let _child = tracing::info_span!("child").entered();
        assert_eq!(&*last_entered_scope.lock().unwrap(), &["root", "child",]);
        let _leaf = tracing::info_span!("leaf").entered();
        assert_eq!(
            &*last_entered_scope.lock().unwrap(),
            &["root", "child", "leaf"]
        );
||||||| 386969ba
impl<'span, R> std::fmt::Debug for FromRoot<'span, R>
where
    R: LookupSpan<'span>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("FromRoot { .. }")
=======
#[cfg(all(test, feature = "registry", feature = "std"))]
mod tests {
    use crate::{
        prelude::*,
        registry::LookupSpan,
        subscribe::{Context, Subscribe},
    };
    use std::sync::{Arc, Mutex};
    use tracing::{span, Collect};

    #[test]
    fn spanref_scope_iteration_order() {
        let last_entered_scope = Arc::new(Mutex::new(Vec::new()));

        #[derive(Default)]
        struct RecordingSubscriber {
            last_entered_scope: Arc<Mutex<Vec<&'static str>>>,
        }

        impl<S> Subscribe<S> for RecordingSubscriber
        where
            S: Collect + for<'lookup> LookupSpan<'lookup>,
        {
            fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
                let span = ctx.span(id).unwrap();
                let scope = span.scope().map(|span| span.name()).collect::<Vec<_>>();
                *self.last_entered_scope.lock().unwrap() = scope;
            }
        }

        let _guard = tracing::collect::set_default(crate::registry().with(RecordingSubscriber {
            last_entered_scope: last_entered_scope.clone(),
        }));

        let _root = tracing::info_span!("root").entered();
        assert_eq!(&*last_entered_scope.lock().unwrap(), &["root"]);
        let _child = tracing::info_span!("child").entered();
        assert_eq!(&*last_entered_scope.lock().unwrap(), &["child", "root"]);
        let _leaf = tracing::info_span!("leaf").entered();
        assert_eq!(
            &*last_entered_scope.lock().unwrap(),
            &["leaf", "child", "root"]
        );
    }

    #[test]
    fn spanref_scope_fromroot_iteration_order() {
        let last_entered_scope = Arc::new(Mutex::new(Vec::new()));

        #[derive(Default)]
        struct RecordingSubscriber {
            last_entered_scope: Arc<Mutex<Vec<&'static str>>>,
        }

        impl<S> Subscribe<S> for RecordingSubscriber
        where
            S: Collect + for<'lookup> LookupSpan<'lookup>,
        {
            fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
                let span = ctx.span(id).unwrap();
                let scope = span
                    .scope()
                    .from_root()
                    .map(|span| span.name())
                    .collect::<Vec<_>>();
                *self.last_entered_scope.lock().unwrap() = scope;
            }
        }

        let _guard = tracing::collect::set_default(crate::registry().with(RecordingSubscriber {
            last_entered_scope: last_entered_scope.clone(),
        }));

        let _root = tracing::info_span!("root").entered();
        assert_eq!(&*last_entered_scope.lock().unwrap(), &["root"]);
        let _child = tracing::info_span!("child").entered();
        assert_eq!(&*last_entered_scope.lock().unwrap(), &["root", "child",]);
        let _leaf = tracing::info_span!("leaf").entered();
        assert_eq!(
            &*last_entered_scope.lock().unwrap(),
            &["root", "child", "leaf"]
        );
>>>>>>> origin/master
    }
}
