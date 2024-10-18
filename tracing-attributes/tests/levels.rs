<<<<<<< HEAD
use tracing::subscriber::with_default;
||||||| 386969ba
mod support;
use support::*;

use tracing::subscriber::with_default;
=======
use tracing::collect::with_default;
>>>>>>> origin/master
use tracing::Level;
use tracing_attributes::instrument;
use tracing_mock::*;

#[test]
fn named_levels() {
    #[instrument(level = "trace")]
    fn trace() {}

    #[instrument(level = "Debug")]
    fn debug() {}

    #[instrument(level = "INFO")]
    fn info() {}

    #[instrument(level = "WARn")]
    fn warn() {}

    #[instrument(level = "eRrOr")]
    fn error() {}
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("trace").at_level(Level::TRACE))
        .enter(expect::span().named("trace").at_level(Level::TRACE))
        .exit(expect::span().named("trace").at_level(Level::TRACE))
        .new_span(expect::span().named("debug").at_level(Level::DEBUG))
        .enter(expect::span().named("debug").at_level(Level::DEBUG))
        .exit(expect::span().named("debug").at_level(Level::DEBUG))
        .new_span(expect::span().named("info").at_level(Level::INFO))
        .enter(expect::span().named("info").at_level(Level::INFO))
        .exit(expect::span().named("info").at_level(Level::INFO))
        .new_span(expect::span().named("warn").at_level(Level::WARN))
        .enter(expect::span().named("warn").at_level(Level::WARN))
        .exit(expect::span().named("warn").at_level(Level::WARN))
        .new_span(expect::span().named("error").at_level(Level::ERROR))
        .enter(expect::span().named("error").at_level(Level::ERROR))
        .exit(expect::span().named("error").at_level(Level::ERROR))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("trace").at_level(Level::TRACE))
        .enter(span::mock().named("trace").at_level(Level::TRACE))
        .exit(span::mock().named("trace").at_level(Level::TRACE))
        .new_span(span::mock().named("debug").at_level(Level::DEBUG))
        .enter(span::mock().named("debug").at_level(Level::DEBUG))
        .exit(span::mock().named("debug").at_level(Level::DEBUG))
        .new_span(span::mock().named("info").at_level(Level::INFO))
        .enter(span::mock().named("info").at_level(Level::INFO))
        .exit(span::mock().named("info").at_level(Level::INFO))
        .new_span(span::mock().named("warn").at_level(Level::WARN))
        .enter(span::mock().named("warn").at_level(Level::WARN))
        .exit(span::mock().named("warn").at_level(Level::WARN))
        .new_span(span::mock().named("error").at_level(Level::ERROR))
        .enter(span::mock().named("error").at_level(Level::ERROR))
        .exit(span::mock().named("error").at_level(Level::ERROR))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("trace").at_level(Level::TRACE))
        .enter(expect::span().named("trace").at_level(Level::TRACE))
        .exit(expect::span().named("trace").at_level(Level::TRACE))
        .new_span(expect::span().named("debug").at_level(Level::DEBUG))
        .enter(expect::span().named("debug").at_level(Level::DEBUG))
        .exit(expect::span().named("debug").at_level(Level::DEBUG))
        .new_span(expect::span().named("info").at_level(Level::INFO))
        .enter(expect::span().named("info").at_level(Level::INFO))
        .exit(expect::span().named("info").at_level(Level::INFO))
        .new_span(expect::span().named("warn").at_level(Level::WARN))
        .enter(expect::span().named("warn").at_level(Level::WARN))
        .exit(expect::span().named("warn").at_level(Level::WARN))
        .new_span(expect::span().named("error").at_level(Level::ERROR))
        .enter(expect::span().named("error").at_level(Level::ERROR))
        .exit(expect::span().named("error").at_level(Level::ERROR))
        .only()
>>>>>>> origin/master
        .run_with_handle();

    with_default(collector, || {
        trace();
        debug();
        info();
        warn();
        error();
    });

    handle.assert_finished();
}

#[test]
fn numeric_levels() {
    #[instrument(level = 1)]
    fn trace() {}

    #[instrument(level = 2)]
    fn debug() {}

    #[instrument(level = 3)]
    fn info() {}

    #[instrument(level = 4)]
    fn warn() {}

    #[instrument(level = 5)]
    fn error() {}
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("trace").at_level(Level::TRACE))
        .enter(expect::span().named("trace").at_level(Level::TRACE))
        .exit(expect::span().named("trace").at_level(Level::TRACE))
        .new_span(expect::span().named("debug").at_level(Level::DEBUG))
        .enter(expect::span().named("debug").at_level(Level::DEBUG))
        .exit(expect::span().named("debug").at_level(Level::DEBUG))
        .new_span(expect::span().named("info").at_level(Level::INFO))
        .enter(expect::span().named("info").at_level(Level::INFO))
        .exit(expect::span().named("info").at_level(Level::INFO))
        .new_span(expect::span().named("warn").at_level(Level::WARN))
        .enter(expect::span().named("warn").at_level(Level::WARN))
        .exit(expect::span().named("warn").at_level(Level::WARN))
        .new_span(expect::span().named("error").at_level(Level::ERROR))
        .enter(expect::span().named("error").at_level(Level::ERROR))
        .exit(expect::span().named("error").at_level(Level::ERROR))
        .only()
        .run_with_handle();

    with_default(subscriber, || {
        trace();
        debug();
        info();
        warn();
        error();
    });

    handle.assert_finished();
}

#[test]
fn enum_levels() {
    #[instrument(level = Level::TRACE)]
    fn trace() {}

    #[instrument(level = Level::DEBUG)]
    fn debug() {}

    #[instrument(level = tracing::Level::INFO)]
    fn info() {}

    #[instrument(level = Level::WARN)]
    fn warn() {}

    #[instrument(level = Level::ERROR)]
    fn error() {}
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("trace").at_level(Level::TRACE))
        .enter(expect::span().named("trace").at_level(Level::TRACE))
        .exit(expect::span().named("trace").at_level(Level::TRACE))
        .new_span(expect::span().named("debug").at_level(Level::DEBUG))
        .enter(expect::span().named("debug").at_level(Level::DEBUG))
        .exit(expect::span().named("debug").at_level(Level::DEBUG))
        .new_span(expect::span().named("info").at_level(Level::INFO))
        .enter(expect::span().named("info").at_level(Level::INFO))
        .exit(expect::span().named("info").at_level(Level::INFO))
        .new_span(expect::span().named("warn").at_level(Level::WARN))
        .enter(expect::span().named("warn").at_level(Level::WARN))
        .exit(expect::span().named("warn").at_level(Level::WARN))
        .new_span(expect::span().named("error").at_level(Level::ERROR))
        .enter(expect::span().named("error").at_level(Level::ERROR))
        .exit(expect::span().named("error").at_level(Level::ERROR))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("trace").at_level(Level::TRACE))
        .enter(span::mock().named("trace").at_level(Level::TRACE))
        .exit(span::mock().named("trace").at_level(Level::TRACE))
        .new_span(span::mock().named("debug").at_level(Level::DEBUG))
        .enter(span::mock().named("debug").at_level(Level::DEBUG))
        .exit(span::mock().named("debug").at_level(Level::DEBUG))
        .new_span(span::mock().named("info").at_level(Level::INFO))
        .enter(span::mock().named("info").at_level(Level::INFO))
        .exit(span::mock().named("info").at_level(Level::INFO))
        .new_span(span::mock().named("warn").at_level(Level::WARN))
        .enter(span::mock().named("warn").at_level(Level::WARN))
        .exit(span::mock().named("warn").at_level(Level::WARN))
        .new_span(span::mock().named("error").at_level(Level::ERROR))
        .enter(span::mock().named("error").at_level(Level::ERROR))
        .exit(span::mock().named("error").at_level(Level::ERROR))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("trace").at_level(Level::TRACE))
        .enter(expect::span().named("trace").at_level(Level::TRACE))
        .exit(expect::span().named("trace").at_level(Level::TRACE))
        .new_span(expect::span().named("debug").at_level(Level::DEBUG))
        .enter(expect::span().named("debug").at_level(Level::DEBUG))
        .exit(expect::span().named("debug").at_level(Level::DEBUG))
        .new_span(expect::span().named("info").at_level(Level::INFO))
        .enter(expect::span().named("info").at_level(Level::INFO))
        .exit(expect::span().named("info").at_level(Level::INFO))
        .new_span(expect::span().named("warn").at_level(Level::WARN))
        .enter(expect::span().named("warn").at_level(Level::WARN))
        .exit(expect::span().named("warn").at_level(Level::WARN))
        .new_span(expect::span().named("error").at_level(Level::ERROR))
        .enter(expect::span().named("error").at_level(Level::ERROR))
        .exit(expect::span().named("error").at_level(Level::ERROR))
        .only()
>>>>>>> origin/master
        .run_with_handle();

    with_default(collector, || {
        trace();
        debug();
        info();
        warn();
        error();
    });

    handle.assert_finished();
}

#[test]
fn enum_levels() {
    #[instrument(level = Level::TRACE)]
    fn trace() {}

    #[instrument(level = Level::DEBUG)]
    fn debug() {}

    #[instrument(level = tracing::Level::INFO)]
    fn info() {}

    #[instrument(level = Level::WARN)]
    fn warn() {}

    #[instrument(level = Level::ERROR)]
    fn error() {}
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("trace").at_level(Level::TRACE))
        .enter(expect::span().named("trace").at_level(Level::TRACE))
        .exit(expect::span().named("trace").at_level(Level::TRACE))
        .new_span(expect::span().named("debug").at_level(Level::DEBUG))
        .enter(expect::span().named("debug").at_level(Level::DEBUG))
        .exit(expect::span().named("debug").at_level(Level::DEBUG))
        .new_span(expect::span().named("info").at_level(Level::INFO))
        .enter(expect::span().named("info").at_level(Level::INFO))
        .exit(expect::span().named("info").at_level(Level::INFO))
        .new_span(expect::span().named("warn").at_level(Level::WARN))
        .enter(expect::span().named("warn").at_level(Level::WARN))
        .exit(expect::span().named("warn").at_level(Level::WARN))
        .new_span(expect::span().named("error").at_level(Level::ERROR))
        .enter(expect::span().named("error").at_level(Level::ERROR))
        .exit(expect::span().named("error").at_level(Level::ERROR))
        .only()
        .run_with_handle();

    with_default(collector, || {
        trace();
        debug();
        info();
        warn();
        error();
    });

    handle.assert_finished();
}
