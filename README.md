Developed with ❤️ by [Tabnine](https://www.tabnine.com/?utm_source=referral&utm_medium=organic&utm_campaign=proc-lock)

# Proc-lock
A simple cross-process locking API.

## Quick Start
### Installation
In your `Cargo.toml` file, add:
```toml
[dependencies]
proc-lock = "*"
```

### Using the API directly
```rust
use proc_lock::{lock, LockPath};

fn main() {
    let lock_path = LockPath::Tmp("my_lock.lock");
    let guard = lock(&lock_path).unwrap();
    // Until `guard` is dropped, this code section is atomic across multiple processes.
    // ...
    drop(guard);
}
```

### Using macros
```rust
use proc_lock::proclock;

fn main() {
 // A lock will be acquired at the beginning of this function, and will be released at the end.
 a_sensitive_function();
}

#[proc_lock(name = "my_lock.lock")]
fn a_sensitive_function() {}
```

### Current status
⚠️This crate is in its early stages, breaking changes are expected in upcoming releases.

### Changelog
- 0.4.0 - Change interdependencies to local for better development process
- 0.3.3 - Minor docs changes
- 0.3.2 - Fix macro compilation
- 0.3.1 - Change docs
- 0.3.0 - [breaking changes] Rename crate to `proc-lock`, and the macro to `proc_lock`
- 0.2.1 - Add a notice about renaming the crate 
- 0.2.0 - [breaking changes] Stop supporting non-blocking macro api

### Contribution
Please refer to [our project](https://github.com/tabnine/proc-lock/projects/1), as well as open issues.

Also feel free to discuss and open PRs regardless :)

### Developement
1. Run tests:
```bash
cargo test
```
2. Run clippy
```bash
cargo clippy --all --all-targets --all-features -- --no-deps -D clippy::pedantic
```
