rust
fn require_sync<T: Sync>(_: T) {}

fn test() -> impl Copy {
    if false { require_sync(test()); }
    123
}
