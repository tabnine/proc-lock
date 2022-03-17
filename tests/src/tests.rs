use proc_lock::proc_lock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

static SOME_TIME: Duration = Duration::from_secs(1);

#[test]
fn nested_calls_to_a_function_with_blocking_macro_annotation_should_block_the_caller_thread() {
    let should_fail = Arc::new(AtomicBool::new(false));

    blocking_lock(|| {
        let should_fail_clone = should_fail.clone();
        thread::spawn(move || {
            blocking_lock(|| {});
            // If this thread is not suspended, this `AtomicBool` will be true - which will later fail the test.
            should_fail_clone.store(true, Ordering::SeqCst);
        });

        thread::sleep(SOME_TIME);

        assert!(
            !should_fail.load(Ordering::SeqCst),
            "thread should have been blocked"
        );
    });
}

#[proc_lock(name = "block.lock")]
fn blocking_lock(f: impl Fn()) {
    f();
}
