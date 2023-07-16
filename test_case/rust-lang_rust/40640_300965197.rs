
type CacheHash<F: Filler> = BoundedHash<F::Key, Slot<<F::Value as IntoFuture>::Future>>;
