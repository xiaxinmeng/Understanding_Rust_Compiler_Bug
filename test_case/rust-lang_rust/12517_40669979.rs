 rust
trait Ord {
    /* four comparison methods (default methods) */
    fn cmp(&self, other: &Self) -> Ordering;
}
