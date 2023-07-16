rust
trait Iterator {
    /* ... */

    fn collect_into<E: Extend<Self::Item>>(self, e: E) -> E {
        e.extend(self);
        e
    }
}
