use std::path::{Path, PathBuf};

/// Represents the path in which the lock should be created.
#[derive(Debug)]
pub enum LockPath<P: AsRef<Path>> {
    /// Indicates that `P` should be created in `std::env::temp_dir()`
    Tmp(P),
    /// Indicates that `P` is an absolute path
    FullPath(P),
}

impl<P: AsRef<Path>> LockPath<P> {
    pub(crate) fn to_path_buf(&self) -> PathBuf {
        match self {
            LockPath::Tmp(path) => std::env::temp_dir().join(path),
            LockPath::FullPath(path) => path.as_ref().to_path_buf(),
        }
    }
}
