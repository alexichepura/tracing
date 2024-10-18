//! Compare to the example given in the documentation for the `std::dbg` macro.
#![deny(rust_2018_idioms)]

use tracing_macros::dbg;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1))
    }
}

fn main() {
<<<<<<< HEAD
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Layer::new());
||||||| 386969ba
    env_logger::Builder::new().parse_filters("trace").init();
    #[allow(deprecated)]
    let subscriber = tracing_log::TraceLogger::new();
=======
    let collector = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
>>>>>>> origin/master

<<<<<<< HEAD
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
    dbg!(factorial(4));
||||||| 386969ba
    tracing::subscriber::with_default(subscriber, || dbg!(factorial(4)));
=======
    tracing::collect::with_default(collector, || dbg!(factorial(4)));
>>>>>>> origin/master
}
