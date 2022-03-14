use crate::{try_lock, LockPath};
use spectral::assert_that;
use spectral::prelude::*;
use std::io::ErrorKind;

#[test]
fn should_return_err_when_file_is_already_locked() {
    let path = LockPath::Tmp("a.lock");

    let guard = try_lock(&path);
    assert_that(&guard).is_ok();

    assert_that(&try_lock(&path))
        .is_err()
        .matches(|e| e.kind() == ErrorKind::WouldBlock);
}

#[test]
fn should_unlock_guard_on_drop() {
    let path = LockPath::Tmp("b.lock");

    assert_that(&try_lock(&path)).is_ok();

    assert_that(&try_lock(&path)).is_ok();
}
