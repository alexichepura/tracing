use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tracing::span::{Attributes, Record};
<<<<<<< HEAD
use tracing::{
    debug, error, info, instrument, span, trace, warn, Subscriber, Event, Id, Level, Metadata,
};
use tracing_mock::*;
||||||| 386969ba
use tracing::{span, Event, Id, Level, Metadata, Subscriber};
=======
use tracing::{
    debug, error, info, instrument, span, trace, warn, Collect, Event, Id, Level, Metadata,
};
use tracing_core::span::Current;
use tracing_test::block_on_future;
>>>>>>> origin/master

struct State {
    last_level: Mutex<Option<Level>>,
}

<<<<<<< HEAD
struct TestSubscriber(Arc<State>);
const EXPECTED_DEBUG: Option<Level> = if cfg!(debug_assertions) {
    Some(Level::DEBUG)
} else {
    None
};
||||||| 386969ba
struct TestSubscriber(Arc<State>);
=======
const EXPECTED_DEBUG: Option<Level> = if cfg!(debug_assertions) {
    Some(Level::DEBUG)
} else {
    None
};
>>>>>>> origin/master

struct TestCollector(Arc<State>);

impl Collect for TestCollector {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn new_span(&self, span: &Attributes) -> Id {
        *self.0.last_level.lock().unwrap() = Some(span.metadata().level().clone());
        span::Id::from_u64(42)
    }

    fn record(&self, _span: &Id, _values: &Record) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event) {
        *self.0.last_level.lock().unwrap() = Some(event.metadata().level().clone());
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}

    fn current_span(&self) -> Current {
        Current::unknown()
    }
}

#[track_caller]
fn last(state: &State, expected: Option<Level>) {
    let lvl = state.last_level.lock().unwrap().take();
    assert_eq!(lvl, expected);
}

#[track_caller]
fn last(state: &State, expected: Option<Level>) {
    let lvl = state.last_level.lock().unwrap().take();
    assert_eq!(lvl, expected);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn test_static_max_level_events() {
    let me = Arc::new(State {
        last_level: Mutex::new(None),
    });
    let a = me.clone();

    tracing::collect::with_default(TestCollector(me), || {
        error!("");
        last(&a, Some(Level::ERROR));
        warn!("");
        last(&a, Some(Level::WARN));
        info!("");
        last(&a, Some(Level::INFO));
        debug!("");
        last(&a, EXPECTED_DEBUG);
        trace!("");
        last(&a, None);
    });
}

<<<<<<< HEAD
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn test_static_max_level_spans() {
    let me = Arc::new(State {
        last_level: Mutex::new(None),
    });
    let a = me.clone();

    tracing::subscriber::with_default(TestSubscriber(me), || {
||||||| 386969ba
=======
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn test_static_max_level_spans() {
    let me = Arc::new(State {
        last_level: Mutex::new(None),
    });
    let a = me.clone();

    tracing::collect::with_default(TestCollector(me), || {
>>>>>>> origin/master
        span!(Level::ERROR, "");
        last(&a, Some(Level::ERROR));
        span!(Level::WARN, "");
        last(&a, Some(Level::WARN));
        span!(Level::INFO, "");
        last(&a, Some(Level::INFO));
        span!(Level::DEBUG, "");
        last(&a, EXPECTED_DEBUG);
        span!(Level::TRACE, "");
        last(&a, None);
    });
}

<<<<<<< HEAD
#[instrument(level = "debug")]
#[inline(never)] // this makes it a bit easier to look at the asm output
fn instrumented_fn() {}

#[instrument(level = "debug")]
async fn instrumented_async_fn() {}

#[allow(clippy::manual_async_fn)]
#[instrument(level = "debug")]
fn instrumented_manual_async() -> impl Future<Output = ()> {
    async move {}
}

#[instrument(level = "debug")]
fn instrumented_manual_box_pin() -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async move {})
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn test_static_max_level_instrument() {
    let me = Arc::new(State {
        last_level: Mutex::new(None),
    });
    let a = me.clone();

    tracing::subscriber::with_default(TestSubscriber(me), || {
        block_on_future(async {
            instrumented_fn();
            last(&a, EXPECTED_DEBUG);

            instrumented_async_fn().await;
            last(&a, EXPECTED_DEBUG);

            instrumented_manual_async().await;
            last(&a, EXPECTED_DEBUG);

            instrumented_manual_box_pin().await;
            last(&a, EXPECTED_DEBUG);
        })
    });
||||||| 386969ba
fn last(state: &State, expected: Option<Level>) {
    let mut lvl = state.last_level.lock().unwrap();
    assert_eq!(*lvl, expected);
    *lvl = None;
=======
#[instrument(level = "debug")]
#[inline(never)] // this makes it a bit easier to look at the asm output
fn instrumented_fn() {}

#[instrument(level = "debug")]
async fn instrumented_async_fn() {}

#[allow(clippy::manual_async_fn)]
#[instrument(level = "debug")]
fn instrumented_manual_async() -> impl Future<Output = ()> {
    async move {}
}

#[instrument(level = "debug")]
fn instrumented_manual_box_pin() -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async move {})
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn test_static_max_level_instrument() {
    let me = Arc::new(State {
        last_level: Mutex::new(None),
    });
    let a = me.clone();

    tracing::collect::with_default(TestCollector(me), || {
        block_on_future(async {
            instrumented_fn();
            last(&a, EXPECTED_DEBUG);

            instrumented_async_fn().await;
            last(&a, EXPECTED_DEBUG);

            instrumented_manual_async().await;
            last(&a, EXPECTED_DEBUG);

            instrumented_manual_box_pin().await;
            last(&a, EXPECTED_DEBUG);
        })
    });
>>>>>>> origin/master
}
