rust
impl<'a, T: Trait<'a>> FnOnce<(&'a str,)> for Func<'_, T> { ... }
