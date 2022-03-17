mod lock_guard;
mod lock_path;
#[cfg(test)]
mod tests;

pub use crate::lock_guard::LockGuard;
pub use crate::lock_path::LockPath;
use fs2::FileExt;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

pub type Result<T> = std::io::Result<T>;

/// Locks the file in a blocking manner.
/// # Errors
/// Propagates any [`std::io::Error`](std::io::Error)s caused by opening / locking the file.
pub fn lock<P: AsRef<Path>>(file_path: &LockPath<P>) -> Result<LockGuard> {
    let lock_file = open_file_for_locking(file_path.to_path_buf())?;
    lock_file.lock_exclusive()?;

    Ok(lock_file.into())
}

/// Locks the file in a non-blocking manner, i.e return an error if the file is locked.
/// # Errors
/// Propagates any [`std::io::Error`](std::io::Error)s caused by opening / locking the file.
pub fn try_lock<P: AsRef<Path>>(file_path: &LockPath<P>) -> Result<LockGuard> {
    let lock_file = open_file_for_locking(file_path.to_path_buf())?;
    lock_file.try_lock_exclusive()?;

    Ok(lock_file.into())
}

fn open_file_for_locking(file_path: PathBuf) -> Result<File> {
    let lock_file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;

    Ok(lock_file)
}
