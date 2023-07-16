rust
impl<T> PrintAnything for T { ... }
impl<T: Integer> PrintAnything for T { ... }
impl<T: Collection> PrintAnything for T { ... }
impl<T: Integer + Collection> PrintAnything for T { ... }
