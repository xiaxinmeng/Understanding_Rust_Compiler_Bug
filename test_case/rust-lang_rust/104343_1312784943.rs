
fn require_sync<T: Sync>(t: T) -> T { t }
fn test() -> impl Copy {
    require_sync(test())
}
