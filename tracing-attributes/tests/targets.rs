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
fn default_target() {}

#[instrument(target = "my_target")]
fn custom_target() {}

mod my_mod {
    use tracing_attributes::instrument;

    pub const MODULE_PATH: &str = module_path!();

    #[instrument]
    pub fn default_target() {}

    #[instrument(target = "my_other_target")]
    pub fn custom_target() {}
}

#[test]
fn default_targets() {
    let (collector, handle) = collector::mock()
        .new_span(
            expect::span()
                .named("default_target")
                .with_target(module_path!()),
        )
        .enter(
            expect::span()
                .named("default_target")
                .with_target(module_path!()),
        )
        .exit(
            expect::span()
                .named("default_target")
                .with_target(module_path!()),
        )
        .new_span(
            expect::span()
                .named("default_target")
                .with_target(my_mod::MODULE_PATH),
        )
        .enter(
            expect::span()
                .named("default_target")
                .with_target(my_mod::MODULE_PATH),
        )
        .exit(
            expect::span()
                .named("default_target")
                .with_target(my_mod::MODULE_PATH),
        )
        .only()
        .run_with_handle();

    with_default(collector, || {
        default_target();
        my_mod::default_target();
    });

    handle.assert_finished();
}

#[test]
fn custom_targets() {
<<<<<<< HEAD
    let (subscriber, handle) = subscriber::mock()
||||||| 386969ba
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("custom_target").with_target("my_target"))
        .enter(span::mock().named("custom_target").with_target("my_target"))
        .exit(span::mock().named("custom_target").with_target("my_target"))
=======
    let (collector, handle) = collector::mock()
>>>>>>> origin/master
        .new_span(
            expect::span()
                .named("custom_target")
                .with_target("my_target"),
        )
        .enter(
            expect::span()
                .named("custom_target")
                .with_target("my_target"),
        )
        .exit(
            expect::span()
                .named("custom_target")
                .with_target("my_target"),
        )
        .new_span(
            expect::span()
                .named("custom_target")
                .with_target("my_other_target"),
        )
        .enter(
            expect::span()
                .named("custom_target")
                .with_target("my_other_target"),
        )
        .exit(
            expect::span()
                .named("custom_target")
                .with_target("my_other_target"),
        )
        .only()
        .run_with_handle();

    with_default(collector, || {
        custom_target();
        my_mod::custom_target();
    });

    handle.assert_finished();
}
