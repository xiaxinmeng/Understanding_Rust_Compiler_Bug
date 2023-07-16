rust
type F = impl FnOnce() -> ();
static L: LazyLock<(), F> = LazyLock::new(|| ());
