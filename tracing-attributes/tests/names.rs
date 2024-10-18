<<<<<<< HEAD
use tracing::subscriber::with_default;
||||||| 386969ba
mod support;
use support::*;

use tracing::subscriber::with_default;
=======
use tracing::collect::with_default;
>>>>>>> origin/master
use tracing_attributes::instrument;
use tracing_mock::*;

#[instrument]
fn default_name() {}

#[instrument(name = "my_name")]
fn custom_name() {}

// XXX: it's weird that we support both of these forms, but apparently we
// managed to release a version that accepts both syntax, so now we have to
// support it! yay!
#[instrument("my_other_name")]
fn custom_name_no_equals() {}

#[test]
fn default_name_test() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("default_name"))
        .enter(expect::span().named("default_name"))
        .exit(expect::span().named("default_name"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("default_name"))
        .enter(span::mock().named("default_name"))
        .exit(span::mock().named("default_name"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("default_name"))
        .enter(expect::span().named("default_name"))
        .exit(expect::span().named("default_name"))
        .only()
>>>>>>> origin/master
        .run_with_handle();

    with_default(collector, || {
        default_name();
    });

    handle.assert_finished();
}

#[test]
fn custom_name_test() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("my_name"))
        .enter(expect::span().named("my_name"))
        .exit(expect::span().named("my_name"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("my_name"))
        .enter(span::mock().named("my_name"))
        .exit(span::mock().named("my_name"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("my_name"))
        .enter(expect::span().named("my_name"))
        .exit(expect::span().named("my_name"))
        .only()
>>>>>>> origin/master
        .run_with_handle();

    with_default(collector, || {
        custom_name();
    });

    handle.assert_finished();
}

#[test]
fn custom_name_no_equals_test() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
        .new_span(expect::span().named("my_other_name"))
        .enter(expect::span().named("my_other_name"))
        .exit(expect::span().named("my_other_name"))
        .only()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("my_other_name"))
        .enter(span::mock().named("my_other_name"))
        .exit(span::mock().named("my_other_name"))
        .done()
=======
    let (collector, handle) = collector::mock()
        .new_span(expect::span().named("my_other_name"))
        .enter(expect::span().named("my_other_name"))
        .exit(expect::span().named("my_other_name"))
        .only()
>>>>>>> origin/master
        .run_with_handle();

    with_default(collector, || {
        custom_name_no_equals();
    });

    handle.assert_finished();
}
