 rust
trait IndexRange<T=usize> { fn start(&self) -> Option<T> }

fn index<R: IndexRange>(r: R) { ... }
