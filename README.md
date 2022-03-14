# Proclock
A simple cross-process locking API.

### Implementation
This crate uses [`fs2`](https://docs.rs/fs2) to exclusively lock files, and provides a convenient API to
use this mechanism for synchronizing between multiple processes.

### Documentation
