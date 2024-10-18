// These tests include field filters with no targets, so they have to go in a
// separate file.
#![cfg(feature = "env-filter")]

<<<<<<< HEAD
use tracing::{self, subscriber::with_default, Level};
use tracing_mock::*;
||||||| 386969ba
mod support;
use self::support::*;
use tracing::{self, subscriber::with_default, Level};
=======
use tracing::{self, collect::with_default, Level};
use tracing_mock::*;
>>>>>>> origin/master
use tracing_subscriber::{filter::EnvFilter, prelude::*};

#[test]
fn same_length_targets() {
    let filter: EnvFilter = "foo=trace,bar=trace".parse().expect("filter should parse");
<<<<<<< HEAD
    let (subscriber, finished) = subscriber::mock()
        .event(expect::event().at_level(Level::TRACE))
        .event(expect::event().at_level(Level::TRACE))
        .only()
||||||| 386969ba
    let (subscriber, finished) = subscriber::mock()
        .event(event::mock().at_level(Level::TRACE))
        .event(event::mock().at_level(Level::TRACE))
        .done()
=======
    let (subscriber, finished) = collector::mock()
        .event(expect::event().at_level(Level::TRACE))
        .event(expect::event().at_level(Level::TRACE))
        .only()
>>>>>>> origin/master
        .run_with_handle();
    let subscriber = subscriber.with(filter);

    with_default(subscriber, || {
        tracing::trace!(target: "foo", "foo");
        tracing::trace!(target: "bar", "bar");
    });

    finished.assert_finished();
}

#[test]
fn same_num_fields_event() {
    let filter: EnvFilter = "[{foo}]=trace,[{bar}]=trace"
        .parse()
        .expect("filter should parse");
    let (subscriber, finished) = collector::mock()
        .event(
            expect::event()
                .at_level(Level::TRACE)
                .with_fields(expect::field("foo")),
        )
        .event(
            expect::event()
                .at_level(Level::TRACE)
                .with_fields(expect::field("bar")),
        )
        .only()
        .run_with_handle();
    let subscriber = subscriber.with(filter);
    with_default(subscriber, || {
        tracing::trace!(foo = 1);
        tracing::trace!(bar = 3);
    });

    finished.assert_finished();
}

#[test]
fn same_num_fields_and_name_len() {
    let filter: EnvFilter = "[foo{bar=1}]=trace,[baz{boz=1}]=trace"
        .parse()
        .expect("filter should parse");
    let (subscriber, finished) = collector::mock()
        .new_span(
            expect::span()
                .named("foo")
                .at_level(Level::TRACE)
<<<<<<< HEAD
                .with_field(expect::field("bar")),
||||||| 386969ba
                .with_field(field::mock("bar")),
=======
                .with_fields(expect::field("bar")),
>>>>>>> origin/master
        )
        .new_span(
            expect::span()
                .named("baz")
                .at_level(Level::TRACE)
<<<<<<< HEAD
                .with_field(expect::field("boz")),
||||||| 386969ba
                .with_field(field::mock("boz")),
=======
                .with_fields(expect::field("boz")),
>>>>>>> origin/master
        )
        .only()
        .run_with_handle();
    let subscriber = subscriber.with(filter);
    with_default(subscriber, || {
        tracing::trace_span!("foo", bar = 1);
        tracing::trace_span!("baz", boz = 1);
    });

    finished.assert_finished();
}
