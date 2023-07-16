plain
    |
460 | impl<T> RwLock<T> {
    |      - this type parameter
...
548 |     pub fn leak(&self) -> &T {
    |                           -- expected `&T` because of return type
552 |         ret
    |         ^^^
    |         |
    |         |
    |         expected `&T`, found type parameter `T`
    |
    = note:   expected reference `&T`
            found type parameter `T`

