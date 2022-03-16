Developed with ❤️ by [Tabnine](https://www.tabnine.com)

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

#[proclock(name = "my_lock.lock", blocking = true)] // default value for `blocking` is `false`
fn a_sensitive_function() {}
```
