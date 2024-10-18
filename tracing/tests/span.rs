// These tests require the thread-local scoped dispatcher, which only works when
// we have a standard library. The behaviour being tested should be the same
// with the standard lib disabled.
#![cfg(feature = "std")]

use std::thread;

use tracing::{
    collect::with_default,
    error_span,
    field::{debug, display},
    Level, Span,
};
use tracing_mock::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn handles_to_the_same_span_are_equal() {
    // Create a mock collector that will return `true` on calls to
    // `Collector::enabled`, so that the spans will be constructed. We
    // won't enter any spans in this test, so the collector won't actually
    // expect to see any spans.
<<<<<<< HEAD
    with_default(subscriber::mock().run(), || {
        let foo1 = tracing::span!(Level::TRACE, "foo");

        // The purpose of this test is to assert that two clones of the same
        // span are equal, so the clone here is kind of the whole point :)
        #[allow(clippy::redundant_clone)]
||||||| 386969ba
    with_default(subscriber::mock().run(), || {
        let foo1 = span!(Level::TRACE, "foo");
=======
    with_default(collector::mock().run(), || {
        let foo1 = tracing::span!(Level::TRACE, "foo");

        // The purpose of this test is to assert that two clones of the same
        // span are equal, so the clone here is kind of the whole point :)
        #[allow(clippy::redundant_clone)]
>>>>>>> origin/master
        let foo2 = foo1.clone();

        // Two handles that point to the same span are equal.
        assert_eq!(foo1, foo2);
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn handles_to_different_spans_are_not_equal() {
    with_default(collector::mock().run(), || {
        // Even though these spans have the same name and fields, they will have
        // differing metadata, since they were created on different lines.
        let foo1 = tracing::span!(Level::TRACE, "foo", bar = 1u64, baz = false);
        let foo2 = tracing::span!(Level::TRACE, "foo", bar = 1u64, baz = false);

        assert_ne!(foo1, foo2);
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn handles_to_different_spans_with_the_same_metadata_are_not_equal() {
    // Every time time this function is called, it will return a _new
    // instance_ of a span with the same metadata, name, and fields.
    fn make_span() -> Span {
        tracing::span!(Level::TRACE, "foo", bar = 1u64, baz = false)
    }

    with_default(collector::mock().run(), || {
        let foo1 = make_span();
        let foo2 = make_span();

        assert_ne!(foo1, foo2);
        // assert_ne!(foo1.data(), foo2.data());
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn spans_always_go_to_the_subscriber_that_tagged_them() {
<<<<<<< HEAD
    let subscriber1 = subscriber::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let subscriber1 = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let collector1 = collector::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run();
    let collector2 = collector::mock().run();

<<<<<<< HEAD
    let foo = with_default(subscriber1, || {
        let foo = tracing::span!(Level::TRACE, "foo");
||||||| 386969ba
    let foo = with_default(subscriber1, || {
        let foo = span!(Level::TRACE, "foo");
=======
    let foo = with_default(collector1, || {
        let foo = tracing::span!(Level::TRACE, "foo");
>>>>>>> origin/master
        foo.in_scope(|| {});
        foo
    });
    // Even though we enter subscriber 2's context, the subscriber that
    // tagged the span should see the enter/exit.
    with_default(collector2, move || foo.in_scope(|| {}));
}

// This gets exempt from testing in wasm because of: `thread::spawn` which is
// not yet possible to do in WASM. There is work going on see:
// <https://rustwasm.github.io/2018/10/24/multithreading-rust-and-wasm.html>
//
// But for now since it's not possible we don't need to test for it :)
#[test]
fn spans_always_go_to_the_subscriber_that_tagged_them_even_across_threads() {
<<<<<<< HEAD
    let subscriber1 = subscriber::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let subscriber1 = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let collector1 = collector::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run();
<<<<<<< HEAD
    let foo = with_default(subscriber1, || {
        let foo = tracing::span!(Level::TRACE, "foo");
||||||| 386969ba
    let foo = with_default(subscriber1, || {
        let foo = span!(Level::TRACE, "foo");
=======
    let foo = with_default(collector1, || {
        let foo = tracing::span!(Level::TRACE, "foo");
>>>>>>> origin/master
        foo.in_scope(|| {});
        foo
    });

    // Even though we enter subscriber 2's context, the subscriber that
    // tagged the span should see the enter/exit.
    thread::spawn(move || {
        with_default(collector::mock().run(), || {
            foo.in_scope(|| {});
        })
    })
    .join()
    .unwrap();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn dropping_a_span_calls_drop_span() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        let span = tracing::span!(Level::TRACE, "foo");
||||||| 386969ba
    with_default(subscriber, || {
        let span = span!(Level::TRACE, "foo");
=======
    with_default(collector, || {
        let span = tracing::span!(Level::TRACE, "foo");
>>>>>>> origin/master
        span.in_scope(|| {});
        drop(span);
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn span_closes_after_event() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .event(event::mock())
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "foo").in_scope(|| {
            tracing::event!(Level::DEBUG, {}, "my tracing::event!");
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "foo").in_scope(|| {
            event!(Level::DEBUG, {}, "my event!");
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "foo").in_scope(|| {
            tracing::event!(Level::DEBUG, {}, "my tracing::event!");
>>>>>>> origin/master
        });
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn new_span_after_event() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .enter(expect::span().named("bar"))
        .exit(expect::span().named("bar"))
        .drop_span(expect::span().named("bar"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .event(event::mock())
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .enter(span::mock().named("bar"))
        .exit(span::mock().named("bar"))
        .drop_span(span::mock().named("bar"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .enter(expect::span().named("bar"))
        .exit(expect::span().named("bar"))
        .drop_span(expect::span().named("bar"))
        .only()
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "foo").in_scope(|| {
            tracing::event!(Level::DEBUG, {}, "my tracing::event!");
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "foo").in_scope(|| {
            event!(Level::DEBUG, {}, "my event!");
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "foo").in_scope(|| {
            tracing::event!(Level::DEBUG, {}, "my tracing::event!");
>>>>>>> origin/master
        });
        tracing::span!(Level::TRACE, "bar").in_scope(|| {});
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn event_outside_of_span() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .event(expect::event())
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .event(event::mock())
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .event(expect::event())
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::debug!("my tracing::event!");
        tracing::span!(Level::TRACE, "foo").in_scope(|| {});
||||||| 386969ba
    with_default(subscriber, || {
        debug!("my event!");
        span!(Level::TRACE, "foo").in_scope(|| {});
=======
    with_default(collector, || {
        tracing::debug!("my tracing::event!");
        tracing::span!(Level::TRACE, "foo").in_scope(|| {});
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn cloning_a_span_calls_clone_span() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .clone_span(expect::span().named("foo"))
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .clone_span(span::mock().named("foo"))
=======
    let (collector, handle) = collector::mock()
        .clone_span(expect::span().named("foo"))
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        let span = tracing::span!(Level::TRACE, "foo");
||||||| 386969ba
    with_default(subscriber, || {
        let span = span!(Level::TRACE, "foo");
=======
    with_default(collector, || {
        let span = tracing::span!(Level::TRACE, "foo");
>>>>>>> origin/master
        // Allow the "redundant" `.clone` since it is used to call into the `.clone_span` hook.
        #[allow(clippy::redundant_clone)]
        let _span2 = span.clone();
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn drop_span_when_exiting_dispatchers_context() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .clone_span(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .clone_span(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
=======
    let (collector, handle) = collector::mock()
        .clone_span(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        let span = tracing::span!(Level::TRACE, "foo");
||||||| 386969ba
    with_default(subscriber, || {
        let span = span!(Level::TRACE, "foo");
=======
    with_default(collector, || {
        let span = tracing::span!(Level::TRACE, "foo");
>>>>>>> origin/master
        let _span2 = span.clone();
        drop(span);
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn clone_and_drop_span_always_go_to_the_subscriber_that_tagged_the_span() {
<<<<<<< HEAD
    let (subscriber1, handle1) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .clone_span(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
||||||| 386969ba
    let (subscriber1, handle1) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .clone_span(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
=======
    let (subscriber1, handle1) = collector::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .clone_span(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    let subscriber2 = subscriber::mock().only().run();
||||||| 386969ba
    let subscriber2 = subscriber::mock().done().run();
=======
    let subscriber2 = collector::mock().only().run();
>>>>>>> origin/master

    let foo = with_default(subscriber1, || {
        let foo = tracing::span!(Level::TRACE, "foo");
        foo.in_scope(|| {});
        foo
    });
    // Even though we enter subscriber 2's context, the subscriber that
    // tagged the span should see the enter/exit.
    with_default(subscriber2, move || {
        let foo2 = foo.clone();
        foo.in_scope(|| {});
        drop(foo);
        drop(foo2);
    });

    handle1.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn span_closes_when_exited() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        let foo = tracing::span!(Level::TRACE, "foo");
||||||| 386969ba
    with_default(subscriber, || {
        let foo = span!(Level::TRACE, "foo");
=======
    with_default(collector, || {
        let foo = tracing::span!(Level::TRACE, "foo");
>>>>>>> origin/master

        foo.in_scope(|| {});

        drop(foo);
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn enter() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .event(event::mock())
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        let foo = tracing::span!(Level::TRACE, "foo");
||||||| 386969ba
    with_default(subscriber, || {
        let foo = span!(Level::TRACE, "foo");
=======
    with_default(collector, || {
        let foo = tracing::span!(Level::TRACE, "foo");
>>>>>>> origin/master
        let _enter = foo.enter();
<<<<<<< HEAD
        tracing::debug!("dropping guard...");
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn entered() {
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
        .run_with_handle();
    with_default(subscriber, || {
        let _span = tracing::span!(Level::TRACE, "foo").entered();
        tracing::debug!("dropping guard...");
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn entered_api() {
    let (subscriber, handle) = subscriber::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
        .run_with_handle();
    with_default(subscriber, || {
        let span = tracing::span!(Level::TRACE, "foo").entered();
        let _derefs_to_span = span.id();
        tracing::debug!("exiting span...");
        let _: Span = span.exit();
||||||| 386969ba
        debug!("dropping guard...");
=======
        tracing::debug!("dropping guard...");
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn entered() {
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
        .run_with_handle();
    with_default(collector, || {
        let _span = tracing::span!(Level::TRACE, "foo").entered();
        tracing::debug!("dropping guard...");
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn entered_api() {
    let (collector, handle) = collector::mock()
        .enter(expect::span().named("foo"))
        .event(expect::event())
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
        .run_with_handle();
    with_default(collector, || {
        let span = tracing::span!(Level::TRACE, "foo").entered();
        let _derefs_to_span = span.id();
        tracing::debug!("exiting span...");
        let _: Span = span.exit();
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn moved_field() {
    let (collector, handle) = collector::mock()
        .new_span(
<<<<<<< HEAD
            expect::span().named("foo").with_field(
                expect::field("bar")
||||||| 386969ba
            span::mock().named("foo").with_field(
                field::mock("bar")
=======
            expect::span().named("foo").with_fields(
                expect::field("bar")
>>>>>>> origin/master
                    .with_value(&display("hello from my span"))
                    .only(),
            ),
        )
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
        .run_with_handle();
    with_default(collector, || {
        let from = "my span";
        let span = tracing::span!(
            Level::TRACE,
            "foo",
            bar = display(format!("hello from {}", from))
        );
        span.in_scope(|| {});
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn dotted_field_name() {
    let (collector, handle) = collector::mock()
        .new_span(
            expect::span()
                .named("foo")
<<<<<<< HEAD
                .with_field(expect::field("fields.bar").with_value(&true).only()),
||||||| 386969ba
                .with_field(field::mock("fields.bar").with_value(&true).only()),
=======
                .with_fields(expect::field("fields.bar").with_value(&true).only()),
>>>>>>> origin/master
        )
        .only()
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "foo", fields.bar = true);
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "foo", fields.bar = true);
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "foo", fields.bar = true);
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn borrowed_field() {
    let (collector, handle) = collector::mock()
        .new_span(
<<<<<<< HEAD
            expect::span().named("foo").with_field(
                expect::field("bar")
||||||| 386969ba
            span::mock().named("foo").with_field(
                field::mock("bar")
=======
            expect::span().named("foo").with_fields(
                expect::field("bar")
>>>>>>> origin/master
                    .with_value(&display("hello from my span"))
                    .only(),
            ),
        )
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
        .run_with_handle();

    with_default(collector, || {
        let from = "my span";
        let mut message = format!("hello from {}", from);
        let span = tracing::span!(Level::TRACE, "foo", bar = display(&message));
        span.in_scope(|| {
            message.insert_str(10, " inside");
        });
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
// If emitting log instrumentation, this gets moved anyway, breaking the test.
#[cfg(not(feature = "log"))]
fn move_field_out_of_struct() {
    use tracing::field::debug;

    #[derive(Debug)]
    struct Position {
        x: f32,
        y: f32,
    }

    let pos = Position {
        x: 3.234,
        y: -1.223,
    };
    let (collector, handle) = collector::mock()
        .new_span(
<<<<<<< HEAD
            expect::span().named("foo").with_field(
                expect::field("x")
||||||| 386969ba
            span::mock().named("foo").with_field(
                field::mock("x")
=======
            expect::span().named("foo").with_fields(
                expect::field("x")
>>>>>>> origin/master
                    .with_value(&debug(3.234))
                    .and(expect::field("y").with_value(&debug(-1.223)))
                    .only(),
            ),
        )
        .new_span(
            expect::span()
                .named("bar")
<<<<<<< HEAD
                .with_field(expect::field("position").with_value(&debug(&pos)).only()),
||||||| 386969ba
                .with_field(field::mock("position").with_value(&debug(&pos)).only()),
=======
                .with_fields(expect::field("position").with_value(&debug(&pos)).only()),
>>>>>>> origin/master
        )
        .run_with_handle();

    with_default(collector, || {
        let pos = Position {
            x: 3.234,
            y: -1.223,
        };
        let foo = tracing::span!(Level::TRACE, "foo", x = debug(pos.x), y = debug(pos.y));
        let bar = tracing::span!(Level::TRACE, "bar", position = debug(pos));
        foo.in_scope(|| {});
        bar.in_scope(|| {});
    });

    handle.assert_finished();
}

<<<<<<< HEAD
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn float_values() {
    let (subscriber, handle) = subscriber::mock()
        .new_span(
            expect::span().named("foo").with_field(
                expect::field("x")
                    .with_value(&3.234)
                    .and(expect::field("y").with_value(&-1.223))
                    .only(),
            ),
        )
        .run_with_handle();

    with_default(subscriber, || {
        let foo = tracing::span!(Level::TRACE, "foo", x = 3.234, y = -1.223);
        foo.in_scope(|| {});
    });

    handle.assert_finished();
}

||||||| 386969ba
=======
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn float_values() {
    let (collector, handle) = collector::mock()
        .new_span(
            expect::span().named("foo").with_fields(
                expect::field("x")
                    .with_value(&3.234)
                    .and(expect::field("y").with_value(&-1.223))
                    .only(),
            ),
        )
        .run_with_handle();

    with_default(collector, || {
        let foo = tracing::span!(Level::TRACE, "foo", x = 3.234, y = -1.223);
        foo.in_scope(|| {});
    });

    handle.assert_finished();
}

>>>>>>> origin/master
// TODO(#1138): determine a new syntax for uninitialized span fields, and
// re-enable these.
/*
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn add_field_after_new_span() {
    let (collector, handle) = subscriber::mock()
        .new_span(
            expect::span()
                .named("foo")
<<<<<<< HEAD
                .with_field(expect::field("bar").with_value(&5)
                .and(expect::field("baz").with_value).only()),
||||||| 386969ba
                .with_field(field::mock("bar").with_value(&5)
                .and(field::mock("baz").with_value).only()),
=======
                .with_fields(expect::field("bar").with_value(&5)
                .and(expect::field("baz").with_value).only()),
>>>>>>> origin/master
        )
        .record(
<<<<<<< HEAD
            expect::span().named("foo"),
            field::expect("baz").with_value(&true).only(),
||||||| 386969ba
            span::mock().named("foo"),
            field::mock("baz").with_value(&true).only(),
=======
            span::mock().named("foo"),
            expect::field("baz").with_value(&true).only(),
>>>>>>> origin/master
        )
<<<<<<< HEAD
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();

    with_default(subscriber, || {
        let span = tracing::span!(Level::TRACE, "foo", bar = 5, baz = false);
        span.record("baz", &true);
        span.in_scope(|| {})
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn add_fields_only_after_new_span() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("foo"))
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("foo"))
=======
    let (collector, handle) = subscriber::mock()
        .new_span(span::mock().named("foo"))
>>>>>>> origin/master
        .record(
<<<<<<< HEAD
            expect::span().named("foo"),
            field::expect("bar").with_value(&5).only(),
||||||| 386969ba
            span::mock().named("foo"),
            field::mock("bar").with_value(&5).only(),
=======
            span::mock().named("foo"),
            expect::field("bar").with_value(&5).only(),
>>>>>>> origin/master
        )
        .record(
<<<<<<< HEAD
            expect::span().named("foo"),
            field::expect("baz").with_value(&true).only(),
||||||| 386969ba
            span::mock().named("foo"),
            field::mock("baz").with_value(&true).only(),
=======
            span::mock().named("foo"),
            expect::field("baz").with_value(&true).only(),
>>>>>>> origin/master
        )
<<<<<<< HEAD
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
||||||| 386969ba
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
=======
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();

    with_default(subscriber, || {
        let span = tracing::span!(Level::TRACE, "foo", bar = _, baz = _);
        span.record("bar", &5);
        span.record("baz", &true);
        span.in_scope(|| {})
    });

    handle.assert_finished();
}
*/

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn record_new_value_for_field() {
    let (collector, handle) = collector::mock()
        .new_span(
<<<<<<< HEAD
            expect::span().named("foo").with_field(
                expect::field("bar")
||||||| 386969ba
            span::mock().named("foo").with_field(
                field::mock("bar")
=======
            expect::span().named("foo").with_fields(
                expect::field("bar")
>>>>>>> origin/master
                    .with_value(&5)
                    .and(expect::field("baz").with_value(&false))
                    .only(),
            ),
        )
        .record(
            expect::span().named("foo"),
            expect::field("baz").with_value(&true).only(),
        )
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        let span = tracing::span!(Level::TRACE, "foo", bar = 5, baz = false);
        span.record("baz", true);
||||||| 386969ba
    with_default(subscriber, || {
        let span = span!(Level::TRACE, "foo", bar = 5, baz = false);
        span.record("baz", &true);
=======
    with_default(collector, || {
        let span = tracing::span!(Level::TRACE, "foo", bar = 5, baz = false);
        span.record("baz", true);
>>>>>>> origin/master
        span.in_scope(|| {})
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn record_new_values_for_fields() {
    let (collector, handle) = collector::mock()
        .new_span(
<<<<<<< HEAD
            expect::span().named("foo").with_field(
                expect::field("bar")
||||||| 386969ba
            span::mock().named("foo").with_field(
                field::mock("bar")
=======
            expect::span().named("foo").with_fields(
                expect::field("bar")
>>>>>>> origin/master
                    .with_value(&4)
                    .and(expect::field("baz").with_value(&false))
                    .only(),
            ),
        )
        .record(
            expect::span().named("foo"),
            expect::field("bar").with_value(&5).only(),
        )
        .record(
            expect::span().named("foo"),
            expect::field("baz").with_value(&true).only(),
        )
        .enter(expect::span().named("foo"))
        .exit(expect::span().named("foo"))
        .drop_span(expect::span().named("foo"))
        .only()
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        let span = tracing::span!(Level::TRACE, "foo", bar = 4, baz = false);
        span.record("bar", 5);
        span.record("baz", true);
||||||| 386969ba
    with_default(subscriber, || {
        let span = span!(Level::TRACE, "foo", bar = 4, baz = false);
        span.record("bar", &5);
        span.record("baz", &true);
=======
    with_default(collector, || {
        let span = tracing::span!(Level::TRACE, "foo", bar = 4, baz = false);
        span.record("bar", 5);
        span.record("baz", true);
>>>>>>> origin/master
        span.in_scope(|| {})
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn new_span_with_target_and_log_level() {
    let (collector, handle) = collector::mock()
        .new_span(
            expect::span()
                .named("foo")
                .with_target("app_span")
                .at_level(Level::DEBUG),
        )
        .only()
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(target: "app_span", Level::DEBUG, "foo");
||||||| 386969ba
    with_default(subscriber, || {
        span!(target: "app_span", Level::DEBUG, "foo");
=======
    with_default(collector, || {
        tracing::span!(target: "app_span", Level::DEBUG, "foo");
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn explicit_root_span_is_root() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("foo").with_explicit_parent(None))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("foo").with_explicit_parent(None))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(
            expect::span()
                .named("foo")
                .with_ancestry(expect::is_explicit_root()),
        )
        .only()
>>>>>>> origin/master
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(parent: None, Level::TRACE, "foo");
||||||| 386969ba
    with_default(subscriber, || {
        span!(parent: None, Level::TRACE, "foo");
=======
    with_default(collector, || {
        tracing::span!(parent: None, Level::TRACE, "foo");
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn explicit_root_span_is_root_regardless_of_ctx() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .new_span(expect::span().named("bar").with_explicit_parent(None))
        .exit(expect::span().named("foo"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
        .new_span(span::mock().named("bar").with_explicit_parent(None))
        .exit(span::mock().named("foo"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
        .new_span(
            expect::span()
                .named("bar")
                .with_ancestry(expect::is_explicit_root()),
        )
        .exit(expect::span().named("foo"))
        .only()
>>>>>>> origin/master
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "foo").in_scope(|| {
            tracing::span!(parent: None, Level::TRACE, "bar");
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "foo").in_scope(|| {
            span!(parent: None, Level::TRACE, "bar");
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "foo").in_scope(|| {
            tracing::span!(parent: None, Level::TRACE, "bar");
>>>>>>> origin/master
        })
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn explicit_child() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("foo"))
        .new_span(
            expect::span()
                .named("bar")
                .with_explicit_parent(Some("foo")),
        )
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("foo"))
        .new_span(span::mock().named("bar").with_explicit_parent(Some("foo")))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("foo"))
        .new_span(
            expect::span()
                .named("bar")
                .with_ancestry(expect::has_explicit_parent("foo")),
        )
        .only()
>>>>>>> origin/master
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        let foo = tracing::span!(Level::TRACE, "foo");
        tracing::span!(parent: foo.id(), Level::TRACE, "bar");
||||||| 386969ba
    with_default(subscriber, || {
        let foo = span!(Level::TRACE, "foo");
        span!(parent: foo.id(), Level::TRACE, "bar");
=======
    with_default(collector, || {
        let foo = tracing::span!(Level::TRACE, "foo");
        tracing::span!(parent: foo.id(), Level::TRACE, "bar");
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn explicit_child_at_levels() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("foo"))
        .new_span(expect::span().named("a").with_explicit_parent(Some("foo")))
        .new_span(expect::span().named("b").with_explicit_parent(Some("foo")))
        .new_span(expect::span().named("c").with_explicit_parent(Some("foo")))
        .new_span(expect::span().named("d").with_explicit_parent(Some("foo")))
        .new_span(expect::span().named("e").with_explicit_parent(Some("foo")))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("foo"))
        .new_span(span::mock().named("a").with_explicit_parent(Some("foo")))
        .new_span(span::mock().named("b").with_explicit_parent(Some("foo")))
        .new_span(span::mock().named("c").with_explicit_parent(Some("foo")))
        .new_span(span::mock().named("d").with_explicit_parent(Some("foo")))
        .new_span(span::mock().named("e").with_explicit_parent(Some("foo")))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("foo"))
        .new_span(
            expect::span()
                .named("a")
                .with_ancestry(expect::has_explicit_parent("foo")),
        )
        .new_span(
            expect::span()
                .named("b")
                .with_ancestry(expect::has_explicit_parent("foo")),
        )
        .new_span(
            expect::span()
                .named("c")
                .with_ancestry(expect::has_explicit_parent("foo")),
        )
        .new_span(
            expect::span()
                .named("d")
                .with_ancestry(expect::has_explicit_parent("foo")),
        )
        .new_span(
            expect::span()
                .named("e")
                .with_ancestry(expect::has_explicit_parent("foo")),
        )
        .only()
>>>>>>> origin/master
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        let foo = tracing::span!(Level::TRACE, "foo");
        tracing::trace_span!(parent: foo.id(), "a");
        tracing::debug_span!(parent: foo.id(), "b");
        tracing::info_span!(parent: foo.id(), "c");
        tracing::warn_span!(parent: foo.id(), "d");
        tracing::error_span!(parent: foo.id(), "e");
||||||| 386969ba
    with_default(subscriber, || {
        let foo = span!(Level::TRACE, "foo");
        trace_span!(parent: foo.id(), "a");
        debug_span!(parent: foo.id(), "b");
        info_span!(parent: foo.id(), "c");
        warn_span!(parent: foo.id(), "d");
        error_span!(parent: foo.id(), "e");
=======
    with_default(collector, || {
        let foo = tracing::span!(Level::TRACE, "foo");
        tracing::trace_span!(parent: foo.id(), "a");
        tracing::debug_span!(parent: foo.id(), "b");
        tracing::info_span!(parent: foo.id(), "c");
        tracing::warn_span!(parent: foo.id(), "d");
        tracing::error_span!(parent: foo.id(), "e");
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn explicit_child_regardless_of_ctx() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("foo"))
        .new_span(expect::span().named("bar"))
        .enter(expect::span().named("bar"))
        .new_span(
            expect::span()
                .named("baz")
                .with_explicit_parent(Some("foo")),
        )
        .exit(expect::span().named("bar"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("foo"))
        .new_span(span::mock().named("bar"))
        .enter(span::mock().named("bar"))
        .new_span(span::mock().named("baz").with_explicit_parent(Some("foo")))
        .exit(span::mock().named("bar"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("foo"))
        .new_span(expect::span().named("bar"))
        .enter(expect::span().named("bar"))
        .new_span(
            expect::span()
                .named("baz")
                .with_ancestry(expect::has_explicit_parent("foo")),
        )
        .exit(expect::span().named("bar"))
        .only()
>>>>>>> origin/master
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        let foo = tracing::span!(Level::TRACE, "foo");
        tracing::span!(Level::TRACE, "bar")
            .in_scope(|| tracing::span!(parent: foo.id(), Level::TRACE, "baz"))
||||||| 386969ba
    with_default(subscriber, || {
        let foo = span!(Level::TRACE, "foo");
        span!(Level::TRACE, "bar").in_scope(|| span!(parent: foo.id(), Level::TRACE, "baz"))
=======
    with_default(collector, || {
        let foo = tracing::span!(Level::TRACE, "foo");
        tracing::span!(Level::TRACE, "bar")
            .in_scope(|| tracing::span!(parent: foo.id(), Level::TRACE, "baz"))
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn contextual_root() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("foo").with_contextual_parent(None))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("foo").with_contextual_parent(None))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(
            expect::span()
                .named("foo")
                .with_ancestry(expect::is_contextual_root()),
        )
        .only()
>>>>>>> origin/master
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "foo");
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "foo");
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "foo");
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn contextual_child() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("foo"))
        .enter(expect::span().named("foo"))
>>>>>>> origin/master
        .new_span(
            expect::span()
                .named("bar")
                .with_ancestry(expect::has_contextual_parent("foo")),
        )
        .exit(expect::span().named("foo"))
        .only()
        .run_with_handle();

<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "foo").in_scope(|| {
            tracing::span!(Level::TRACE, "bar");
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "foo").in_scope(|| {
            span!(Level::TRACE, "bar");
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "foo").in_scope(|| {
            tracing::span!(Level::TRACE, "bar");
>>>>>>> origin/master
        })
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn display_shorthand() {
    let (collector, handle) = collector::mock()
        .new_span(
<<<<<<< HEAD
            expect::span().named("my_span").with_field(
                expect::field("my_field")
||||||| 386969ba
            span::mock().named("my_span").with_field(
                field::mock("my_field")
=======
            expect::span().named("my_span").with_fields(
                expect::field("my_field")
>>>>>>> origin/master
                    .with_value(&display("hello world"))
                    .only(),
            ),
        )
        .only()
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "my_span", my_field = %"hello world");
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "my_span", my_field = %"hello world");
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "my_span", my_field = %"hello world");
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn debug_shorthand() {
    let (collector, handle) = collector::mock()
        .new_span(
<<<<<<< HEAD
            expect::span().named("my_span").with_field(
                expect::field("my_field")
||||||| 386969ba
            span::mock().named("my_span").with_field(
                field::mock("my_field")
=======
            expect::span().named("my_span").with_fields(
                expect::field("my_field")
>>>>>>> origin/master
                    .with_value(&debug("hello world"))
                    .only(),
            ),
        )
        .only()
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "my_span", my_field = ?"hello world");
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "my_span", my_field = ?"hello world");
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "my_span", my_field = ?"hello world");
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn both_shorthands() {
    let (collector, handle) = collector::mock()
        .new_span(
<<<<<<< HEAD
            expect::span().named("my_span").with_field(
                expect::field("display_field")
||||||| 386969ba
            span::mock().named("my_span").with_field(
                field::mock("display_field")
=======
            expect::span().named("my_span").with_fields(
                expect::field("display_field")
>>>>>>> origin/master
                    .with_value(&display("hello world"))
                    .and(expect::field("debug_field").with_value(&debug("hello world")))
                    .only(),
            ),
        )
        .only()
        .run_with_handle();
<<<<<<< HEAD
    with_default(subscriber, || {
        tracing::span!(Level::TRACE, "my_span", display_field = %"hello world", debug_field = ?"hello world");
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn constant_field_name() {
    let (subscriber, handle) = subscriber::mock()
        .new_span(
            expect::span().named("my_span").with_field(
                expect::field("foo")
                    .with_value(&"bar")
                    .and(expect::field("constant string").with_value(&"also works"))
                    .and(expect::field("foo.bar").with_value(&"baz"))
                    .only(),
            ),
        )
        .only()
        .run_with_handle();

    with_default(subscriber, || {
        const FOO: &str = "foo";
        tracing::span!(
            Level::TRACE,
            "my_span",
            { std::convert::identity(FOO) } = "bar",
            { "constant string" } = "also works",
            foo.bar = "baz",
        );
||||||| 386969ba
    with_default(subscriber, || {
        span!(Level::TRACE, "my_span", display_field = %"hello world", debug_field = ?"hello world");
=======
    with_default(collector, || {
        tracing::span!(Level::TRACE, "my_span", display_field = %"hello world", debug_field = ?"hello world");
>>>>>>> origin/master
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn constant_field_name() {
    let (collector, handle) = collector::mock()
        .new_span(
            expect::span().named("my_span").with_fields(
                expect::field("foo")
                    .with_value(&"bar")
                    .and(expect::field("constant string").with_value(&"also works"))
                    .and(expect::field("foo.bar").with_value(&"baz"))
                    .only(),
            ),
        )
        .only()
        .run_with_handle();

    with_default(collector, || {
        const FOO: &str = "foo";
        tracing::span!(
            Level::TRACE,
            "my_span",
            { std::convert::identity(FOO) } = "bar",
            { "constant string" } = "also works",
            foo.bar = "baz",
        );
    });

    handle.assert_finished();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn keyword_ident_in_field_name_span_macro() {
    #[derive(Debug)]
    struct Foo;

    let (collector, handle) = collector::mock()
        .new_span(expect::span().with_fields(expect::field("self").with_value(&debug(Foo)).only()))
        .only()
        .run_with_handle();

    with_default(collector, || {
        error_span!("span", self = ?Foo);
    });
    handle.assert_finished();
}
