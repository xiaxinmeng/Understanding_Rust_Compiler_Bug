rust
> ...
> #[cfg_attr(bootstrap, lang = "manually_drop")]
> #[cfg_attr(not(bootstrap), manually_drop)]
> ...
> pub struct ManuallyDrop<T: ?Sized> { ... }
> 