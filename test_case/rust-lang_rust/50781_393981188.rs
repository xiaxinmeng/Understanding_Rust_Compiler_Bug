rust
// Crate A
pub trait X<T> {
    fn foo(&self) where Self: PartialEq<T>;
}

// Crate B
struct Mine;

impl X<Mine> for Mine {
    fn foo(&self) {}
}

impl PartialEq<Mine> for X<Mine> { fn eq(&self, _: &Mine) -> bool { true } }

pub fn main() {
    <X<Mine> as X<Mine>>::foo(&Mine); // Segfault at opt-level 0, SIGILL otherwise.
}
