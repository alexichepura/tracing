//! Core primitives for `tracing`.
//!
//! [`tracing`] is a framework for instrumenting Rust programs to collect
//! structured, event-based diagnostic information. This crate defines the core
//! primitives of `tracing`.
//!
//! This crate provides:
//!
//! * [`span::Id`] identifies a span within the execution of a program.
//!
//! * [`Event`] represents a single event within a trace.
//!
//! * [`Collect`], the trait implemented to collect trace data.
//!
//! * [`Metadata`] and [`Callsite`] provide information describing spans and
//!   `Event`s.
//!
//! * [`Field`], [`FieldSet`], [`Value`], and [`ValueSet`] represent the
//!   structured data attached to a span.
//!
//! * [`Dispatch`] allows spans and events to be dispatched to collectors.
//!
//! In addition, it defines the global callsite registry and per-thread current
//! dispatcher which other components of the tracing system rely on.
//!
//! *Compiler support: [requires `rustc` 1.63+][msrv]*
//!
//! [msrv]: #supported-rust-versions
//!
//! ## Usage
//!
//! Application authors will typically not use this crate directly. Instead,
//! they will use the [`tracing`] crate, which provides a much more
//! fully-featured API. However, this crate's API will change very infrequently,
//! so it may be used when dependencies must be very stable.
//!
//! Collector implementations may depend on `tracing-core` rather than
//! `tracing`, as the additional APIs provided by `tracing` are primarily useful
//! for instrumenting libraries and applications, and are generally not
//! necessary for collector implementations.
//!
//! The [`tokio-rs/tracing`] repository contains less stable crates designed to
//! be used with the `tracing` ecosystem. It includes a collection of
//! collector implementations, as well as utility and adapter crates.
//!
//! ### `no_std` Support
//!
//! In embedded systems and other bare-metal applications, `tracing-core` can be
//! used without requiring the Rust standard library, although some features are
//! disabled.
//!
//! The dependency on the standard library is controlled by two crate feature
//! flags, "std", which enables the dependency on [`libstd`], and "alloc", which
//! enables the dependency on [`liballoc`] (and is enabled by the "std"
//! feature). These features are enabled by default, but `no_std` users can
//! disable them using:
//!
//! ```toml
//! # Cargo.toml
//! tracing-core = { version = "0.2", default-features = false }
//! ```
//!
//! To enable `liballoc` but not `std`, use:
//!
//! ```toml
//! # Cargo.toml
//! tracing-core = { version = "0.2", default-features = false, features = ["alloc"] }
//! ```
//!
//! When both the "std" and "alloc" feature flags are disabled, `tracing-core`
//! will not make any dynamic memory allocations at runtime, and does not
//! require a global memory allocator.
//!
//! The "alloc" feature is required to enable the [`Dispatch::new`] function,
//! which requires dynamic memory allocation to construct a collector trait
//! object at runtime. When liballoc is disabled, new `Dispatch`s may still be
//! created from `&'static dyn Collect` references, using
//! [`Dispatch::from_static`].
//!
//! The "std" feature is required to enable the following features:
//!
//! * Per-thread scoped trace dispatchers ([`Dispatch::set_default`] and
//!   [`with_default`]. Since setting a thread-local dispatcher inherently
//!   requires a concept of threads to be available, this API is not possible
//!   without the standard library.
//! * Support for [constructing `Value`s from types implementing
//!   `std::error::Error`][err]. Since the `Error` trait is defined in `std`,
//!   it's not possible to provide this feature without `std`.
//!
//! All other features of `tracing-core` should behave identically with and
//! without `std` and `alloc`.
//!
//! [`libstd`]: std
//! [`Dispatch::new`]: crate::dispatch::Dispatch::new
//! [`Dispatch::from_static`]: crate::dispatch::Dispatch::from_static
//! [`Dispatch::set_default`]: crate::dispatch::set_default
//! [`with_default`]: crate::dispatch::with_default
//! [err]: crate::field::Visit::record_error
//!
//! ## Crate Feature Flags
//!
//! The following crate [feature flags] are available:
//!
//! * `std`: Depend on the Rust standard library (enabled by default).
//! * `alloc`: Depend on [`liballoc`] (enabled by "std").
//!
<<<<<<< HEAD
//!   `no_std` users may disable this feature with `default-features = false`:
//!
//!   ```toml
//!   [dependencies]
//!   tracing-core = { version = "0.1.22", default-features = false }
//!   ```
//!
//!   **Note**:`tracing-core`'s `no_std` support requires `liballoc`.
||||||| 386969ba
//!   `no_std` users may disable this feature with `default-features = false`:
//!
//!   ```toml
//!   [dependencies]
//!   tracing-core = { version = "0.1.17", default-features = false }
//!   ```
//!
//!   **Note**:`tracing-core`'s `no_std` support requires `liballoc`.
=======
//! [`liballoc`]: alloc
>>>>>>> origin/master
//!
//! ### Unstable Features
//!
//! These feature flags enable **unstable** features. The public API may break in 0.1.x
//! releases. To enable these features, the `--cfg tracing_unstable` must be passed to
//! `rustc` when compiling.
//!
//! The following unstable feature flags are currently available:
//!
//! * `valuable`: Enables support for recording [field values] using the
//!   [`valuable`] crate.
//!
//! #### Enabling Unstable Features
//!
//! The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
//! env variable when running `cargo` commands:
//!
//! ```shell
//! RUSTFLAGS="--cfg tracing_unstable" cargo build
//! ```
//! Alternatively, the following can be added to the `.cargo/config` file in a
//! project to automatically enable the cfg flag for that project:
//!
//! ```toml
//! [build]
//! rustflags = ["--cfg", "tracing_unstable"]
//! ```
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//! [field values]: crate::field
//! [`valuable`]: https://crates.io/crates/valuable
//!
//! ## Supported Rust Versions
//!
//! Tracing is built against the latest stable release. The minimum supported
//! version is 1.63. The current Tracing version is not guaranteed to build on
//! Rust versions earlier than the minimum supported version.
//!
//! Tracing follows the same compiler support policies as the rest of the Tokio
//! project. The current stable Rust compiler and the three most recent minor
//! versions before it will always be supported. For example, if the current
//! stable compiler version is 1.69, the minimum supported version will not be
//! increased past 1.66, three minor versions prior. Increasing the minimum
//! supported compiler version is not considered a semver breaking change as
//! long as doing so complies with this policy.
//!
//!
<<<<<<< HEAD
//! [`span::Id`]: span::Id
//! [`Event`]: event::Event
//! [`Subscriber`]: subscriber::Subscriber
//! [`Metadata`]: metadata::Metadata
//! [`Callsite`]: callsite::Callsite
//! [`Field`]: field::Field
//! [`FieldSet`]: field::FieldSet
//! [`Value`]: field::Value
//! [`ValueSet`]: field::ValueSet
//! [`Dispatch`]: dispatcher::Dispatch
||||||| 386969ba
//! [`span::Id`]: span/struct.Id.html
//! [`Event`]: event/struct.Event.html
//! [`Subscriber`]: subscriber/trait.Subscriber.html
//! [`Metadata`]: metadata/struct.Metadata.html
//! [`Callsite`]: callsite/trait.Callsite.html
//! [`Field`]: field/struct.Field.html
//! [`FieldSet`]: field/struct.FieldSet.html
//! [`Value`]: field/trait.Value.html
//! [`ValueSet`]: field/struct.ValueSet.html
//! [`Dispatch`]: dispatcher/struct.Dispatch.html
=======
//! [`Event`]: event::Event
//! [`Collect`]: collect::Collect
//! [`Metadata`]: metadata::Metadata
//! [`Callsite`]: callsite::Callsite
//! [`Field`]: field::Field
//! [`FieldSet`]: field::FieldSet
//! [`Value`]: field::Value
//! [`ValueSet`]: field::ValueSet
//! [`Dispatch`]: dispatch::Dispatch
>>>>>>> origin/master
//! [`tokio-rs/tracing`]: https://github.com/tokio-rs/tracing
//! [`tracing`]: https://crates.io/crates/tracing
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/logo-type.png",
    html_favicon_url = "https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/favicon.ico",
    issue_tracker_base_url = "https://github.com/tokio-rs/tracing/issues/"
)]
#![cfg_attr(not(feature = "std"), no_std)]
<<<<<<< HEAD
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::broken_intra_doc_links))]
||||||| 386969ba
#![cfg_attr(docsrs, feature(doc_cfg), deny(broken_intra_doc_links))]
=======
#![cfg_attr(docsrs, feature(doc_cfg))]
>>>>>>> origin/master
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_interfaces,
    private_bounds,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(hidden)]
pub mod __macro_support {
    // Re-export the `core` functions that are used in macros. This allows
    // a crate to be named `core` and avoid name clashes.
    // See here: https://github.com/tokio-rs/tracing/issues/2761
    pub use core::{file, line, module_path, option::Option};
}

/// Statically constructs an [`Identifier`] for the provided [`Callsite`].
///
<<<<<<< HEAD
/// This may be used in contexts such as static initializers.
||||||| 386969ba
/// This may be used in contexts, such as static initializers, where the
/// [`Callsite::id`] function is not currently usable.
=======
/// This may be used in contexts, such as static initializers, where the
/// [`Metadata::callsite`] function is not currently usable.
>>>>>>> origin/master
///
/// For example:
/// ```rust
<<<<<<< HEAD
/// use tracing_core::{callsite, identify_callsite};
/// # use tracing_core::{Metadata, subscriber::Interest};
||||||| 386969ba
/// # #[macro_use]
/// # extern crate tracing_core;
/// use tracing_core::callsite;
/// # use tracing_core::{Metadata, subscriber::Interest};
=======
/// use tracing_core::{callsite, identify_callsite};
/// # use tracing_core::{Metadata, collect::Interest};
>>>>>>> origin/master
/// # fn main() {
/// pub struct MyCallsite {
///    // ...
/// }
/// impl callsite::Callsite for MyCallsite {
/// # fn set_interest(&self, _: Interest) { unimplemented!() }
/// # fn metadata(&self) -> &Metadata { unimplemented!() }
///     // ...
/// }
///
/// static CALLSITE: MyCallsite = MyCallsite {
///     // ...
/// };
///
/// static CALLSITE_ID: callsite::Identifier = identify_callsite!(&CALLSITE);
/// # }
/// ```
///
<<<<<<< HEAD
/// [`Identifier`]: callsite::Identifier
/// [`Callsite`]: callsite::Callsite
||||||| 386969ba
/// [`Identifier`]: callsite/struct.Identifier.html
/// [`Callsite`]: callsite/trait.Callsite.html
/// [`Callsite::id`]: callsite/trait.Callsite.html#method.id
=======
/// [`Identifier`]: callsite::Identifier
/// [`Callsite`]: callsite::Callsite
/// [`Metadata::callsite`]: metadata::Metadata::callsite
>>>>>>> origin/master
#[macro_export]
macro_rules! identify_callsite {
    ($callsite:expr) => {
        $crate::callsite::Identifier($callsite)
    };
}

/// Statically constructs new span [metadata].
///
/// /// For example:
/// ```rust
<<<<<<< HEAD
/// # use tracing_core::{callsite::Callsite, subscriber::Interest};
/// use tracing_core::metadata;
||||||| 386969ba
/// # #[macro_use]
/// # extern crate tracing_core;
/// # use tracing_core::{callsite::Callsite, subscriber::Interest};
=======
/// # use tracing_core::{callsite::Callsite, collect::Interest};
/// use tracing_core::metadata;
>>>>>>> origin/master
/// use tracing_core::metadata::{Kind, Level, Metadata};
/// # fn main() {
/// # pub struct MyCallsite { }
/// # impl Callsite for MyCallsite {
/// # fn set_interest(&self, _: Interest) { unimplemented!() }
/// # fn metadata(&self) -> &Metadata { unimplemented!() }
/// # }
/// #
/// static FOO_CALLSITE: MyCallsite = MyCallsite {
///     // ...
/// };
///
/// static FOO_METADATA: Metadata = metadata!{
///     name: "foo",
///     target: module_path!(),
///     level: Level::DEBUG,
///     fields: &["bar", "baz"],
///     callsite: &FOO_CALLSITE,
///     kind: Kind::SPAN,
/// };
/// # }
/// ```
///
/// [metadata]: metadata::Metadata
/// [`Metadata::new`]: metadata::Metadata::new
#[macro_export]
macro_rules! metadata {
    (
        name: $name:expr,
        target: $target:expr,
        level: $level:expr,
        fields: $fields:expr,
        callsite: $callsite:expr,
        kind: $kind:expr
    ) => {
        $crate::metadata! {
            name: $name,
            target: $target,
            level: $level,
            fields: $fields,
            callsite: $callsite,
            kind: $kind,
        }
    };
    (
        name: $name:expr,
        target: $target:expr,
        level: $level:expr,
        fields: $fields:expr,
        callsite: $callsite:expr,
        kind: $kind:expr,
    ) => {
        $crate::metadata::Metadata::new(
            $name,
            $target,
            $level,
<<<<<<< HEAD
            ::core::option::Option::Some(file!()),
            ::core::option::Option::Some(line!()),
            ::core::option::Option::Some(module_path!()),
||||||| 386969ba
            Some(file!()),
            Some(line!()),
            Some(module_path!()),
=======
            $crate::__macro_support::Option::Some($crate::__macro_support::file!()),
            $crate::__macro_support::Option::Some($crate::__macro_support::line!()),
            $crate::__macro_support::Option::Some($crate::__macro_support::module_path!()),
>>>>>>> origin/master
            $crate::field::FieldSet::new($fields, $crate::identify_callsite!($callsite)),
            $kind,
        )
    };
}

<<<<<<< HEAD
pub(crate) mod lazy;

// Trimmed-down vendored version of spin 0.5.2 (0387621)
// Dependency of no_std lazy_static, not required in a std build
#[cfg(not(feature = "std"))]
pub(crate) mod spin;

||||||| 386969ba
// std uses lazy_static from crates.io
#[cfg(feature = "std")]
#[macro_use]
extern crate lazy_static;

// no_std uses vendored version of lazy_static 1.4.0 (4216696) with spin
// This can conflict when included in a project already using std lazy_static
// Remove this module when cargo enables specifying dependencies for no_std
#[cfg(not(feature = "std"))]
#[macro_use]
mod lazy_static;

// Trimmed-down vendored version of spin 0.5.2 (0387621)
// Dependency of no_std lazy_static, not required in a std build
#[cfg(not(feature = "std"))]
pub(crate) mod spin;

=======
// Facade module: `no_std` uses spinlocks, `std` uses the mutexes in the standard library
>>>>>>> origin/master
#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub type Once = crate::spin::Once<()>;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use std::sync::Once;

#[cfg(not(feature = "std"))]
// Trimmed-down vendored version of spin 0.5.2 (0387621)
// Required for `Once` in `no_std` builds.
pub(crate) mod spin;

pub mod callsite;
pub mod collect;
pub mod dispatch;
pub mod event;
pub mod field;
pub mod metadata;
mod parent;
pub mod span;

#[doc(inline)]
pub use self::{
    callsite::Callsite,
    collect::Collect,
    dispatch::Dispatch,
    event::Event,
    field::Field,
    metadata::{Level, LevelFilter, Metadata},
};

pub use self::{collect::Interest, metadata::Kind};

mod sealed {
    pub trait Sealed {}
}
