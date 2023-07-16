
#![feature(staged_api)]
#![staged_api]

#[stable]
struct Foo;

#[deprecated(since = "1.0.0", reason = "renamed to LinkedList")]
#[unstable(feature = "collections")]
pub use Foo as Bar;

fn main() {
    Bar::new();
}
