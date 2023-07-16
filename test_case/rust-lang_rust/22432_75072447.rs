 rust
use std::sync::MUTEX_INIT;
fn foo() {
    (0..1).map(|_| MUTEX_INIT).count();
}
