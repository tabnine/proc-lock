//! ---
//! # PLEASE NOTE: THIS CRATE HAS BEEN RENAMED
//!
//! It used to be `proclock`, but it's been renamed to [proc-lock](https://crates.io/crates/proc-lock).
//!
//! Please update your dependencies to receive newer versions.
//!
//! ---
//!
//! A simple cross-process locking API.
//! # Implementation
//! This crate uses [`fs2`](https://docs.rs/fs2) to exclusively lock files, and provides a convenient API to
//! use this mechanism for synchronizing between multiple processes.
//!
//! # Quick Start
//!
//! ### Installation
//! In your `Cargo.toml` file, add:
//! ```toml
//! [dependencies]
//! proclock = "*"
//! ```
//!
//! ### Using the API directly
//! ```rust
//! use proclock::{lock, LockPath};
//!
//! let lock_path = LockPath::Tmp("my_lock.lock");
//! let guard = lock(&lock_path).unwrap();
//! // Until `guard` is dropped, this code section is atomic across multiple processes.
//! // ...
//! drop(guard);
//! ```
//!
//! ### Using macros
//! ```rust
//! use proclock::proclock;
//!
//! fn main() {
//!     // A lock will be acquired at the beginning of this function, and will be released at the end.
//!     a_sensitive_function();
//! }
//!
//! #[proclock(name = "my_lock.lock")]
//! fn a_sensitive_function() {}
//! ```

#[cfg(test)]
mod tests;

pub use proclock_api::*;
pub use proclock_macro::*;
