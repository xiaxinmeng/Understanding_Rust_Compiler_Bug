
#![feature(staged_api)]
#![staged_api]

#[stable(feature = "rust1", since = "1.0.0")]
pub struct Foo;

#[deprecated(since = "1.0.0", reason = "renamed to LinkedList")]
#[unstable(feature = "collections", issue = "0")]
pub use Foo as Bar;

fn main() {
    let bar = Bar;
}
