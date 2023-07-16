rust
use std::sync::Once;

static ONCE: Once = Once::new();

fn main() {
    ONCE.call_once(|| ONCE.call_once(|| {}));
}
