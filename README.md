Developed with ❤️ by [Tabnine](https://www.tabnine.com)

---
# PLEASE NOTE: THIS CRATE HAS BEEN RENAMED

It used to be `proclock`, but it's been renamed to `proc-lock`.

Please update your dependencies to receive newer versions.

---

# Proclock
A simple cross-process locking API.

## Quick Start
### Installation
In your `Cargo.toml` file, add:
```toml
[dependencies]
proclock = "*"
```

### Using the API directly
```rust
use proclock::{lock, LockPath};

let lock_path = LockPath::Tmp("my_lock.lock");
let guard = lock(&lock_path).unwrap();
// Until `guard` is dropped, this code section is atomic across multiple processes.
// ...
drop(guard);
```

### Using macros
```rust
use proclock::proclock;

fn main() {
 // A lock will be acquired at the beginning of this function, and will be released at the end.
 a_sensitive_function();
}

#[proclock(name = "my_lock.lock")]
fn a_sensitive_function() {}
```

### Current status
⚠️This crate is in its early stages, breaking changes are expected in upcoming releases.

### Changelist
- 0.2.1 - Add a notice about renaming the crate 
- 0.2.0 - [breaking changes] Stop supporting non-blocking macro api