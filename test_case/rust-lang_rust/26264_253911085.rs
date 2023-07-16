 rust
pub use tristate::TriState as Spam;

trait Classify {
    fn classify(&self) -> Spam;
}

impl Classify for Message { /* ... */ }

// ...

match message.classify() {
    Spam::Yes     => /* ... */,
    Spam::No      => /* ... */,
    Spam::Unknown => /* ... */
}
