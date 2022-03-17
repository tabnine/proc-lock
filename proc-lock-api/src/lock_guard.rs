use fs2::FileExt;
use std::fs::File;

/// This struct releases the lock on `Drop`, similar to [`MutexGuard`](std::sync::MutexGuard`)'s behaviour
#[derive(Debug)]
pub struct LockGuard {
    /// The file on which the lock has been performed
    inner: File,
}

impl From<File> for LockGuard {
    fn from(inner: File) -> Self {
        LockGuard { inner }
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _unlock_result = self.inner.unlock();
    }
}
