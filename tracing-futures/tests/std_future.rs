use std::{future::Future, pin::Pin, task};

use futures::FutureExt as _;
use tracing::Instrument;
<<<<<<< HEAD
use tracing::{subscriber::with_default, Level};
use tracing_mock::*;
||||||| 386969ba
use tracing::{subscriber::with_default, Level};
=======
use tracing::{collect::with_default, Level};
use tracing_mock::{collector, expect};
use tracing_test::{block_on_future, PollN};
>>>>>>> origin/master

#[test]
fn enter_exit_is_reasonable() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();
    with_default(collector, || {
        let future = PollN::new_ok(2).instrument(tracing::span!(Level::TRACE, "foo"));
        block_on_future(future).unwrap();
    });
    handle.assert_finished();
}

#[test]
fn error_ends_span() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();
    with_default(collector, || {
        let future = PollN::new_err(2).instrument(tracing::span!(Level::TRACE, "foo"));
        block_on_future(future).unwrap_err();
    });
    handle.assert_finished();
}
<<<<<<< HEAD

#[test]
fn span_on_drop() {
    #[derive(Clone, Debug)]
    struct AssertSpanOnDrop;

    impl Drop for AssertSpanOnDrop {
        fn drop(&mut self) {
            tracing::info!("Drop");
        }
    }

    struct Fut(Option<AssertSpanOnDrop>);

    impl Future for Fut {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _: &mut task::Context<'_>) -> task::Poll<Self::Output> {
            self.set(Fut(None));
            task::Poll::Ready(())
        }
    }

    let subscriber = subscriber::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event().at_level(Level::INFO))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .enter(expect::span().named("bar"))
        .event(expect::event().at_level(Level::INFO))
        .exit(expect::span().named("bar"))
        .drop_span(expect::span().named("bar"))
        .only()
        .run();

    with_default(subscriber, || {
        // polled once
        Fut(Some(AssertSpanOnDrop))
            .instrument(tracing::span!(Level::TRACE, "foo"))
            .now_or_never()
            .unwrap();

        // never polled
        drop(Fut(Some(AssertSpanOnDrop)).instrument(tracing::span!(Level::TRACE, "bar")));
    });
}
||||||| 386969ba
=======

#[test]
fn span_on_drop() {
    #[derive(Clone, Debug)]
    struct AssertSpanOnDrop;

    impl Drop for AssertSpanOnDrop {
        fn drop(&mut self) {
            tracing::info!("Drop");
        }
    }

    #[allow(dead_code)] // Field unused, but logs on `Drop`
    struct Fut(Option<AssertSpanOnDrop>);

    impl Future for Fut {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _: &mut task::Context<'_>) -> task::Poll<Self::Output> {
            self.set(Fut(None));
            task::Poll::Ready(())
        }
    }

    let collector = collector::mock()
        .enter(expect::span().named("foo"))
        .event(
            expect::event()
                .with_ancestry(expect::has_contextual_parent("foo"))
                .at_level(Level::INFO),
        )
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .enter(expect::span().named("bar"))
        .event(
            expect::event()
                .with_ancestry(expect::has_contextual_parent("bar"))
                .at_level(Level::INFO),
        )
        .exit(expect::span().named("bar"))
        .drop_span(expect::span().named("bar"))
        .only()
        .run();

    with_default(collector, || {
        // polled once
        Fut(Some(AssertSpanOnDrop))
            .instrument(tracing::span!(Level::TRACE, "foo"))
            .now_or_never()
            .unwrap();

        // never polled
        drop(Fut(Some(AssertSpanOnDrop)).instrument(tracing::span!(Level::TRACE, "bar")));
    });
}
>>>>>>> origin/master
