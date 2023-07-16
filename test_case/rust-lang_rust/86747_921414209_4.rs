rust
// Doesn't trigger a warning.
fn returns_a_guard() -> impl Drop { ... }
