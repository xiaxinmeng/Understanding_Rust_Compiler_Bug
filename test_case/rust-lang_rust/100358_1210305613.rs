rust
use parking_lot::{Mutex, const_mutex};

static BOOLEAN: Mutex<bool> = const_mutex(false);

fn main() {
  BOOLEAN.lock().toggle();
  // The above is far more ergonomic than
  *BOOLEAN.lock() = BOOLEAN.lock().not();
  // or another implementation which reaches the same goal
}
