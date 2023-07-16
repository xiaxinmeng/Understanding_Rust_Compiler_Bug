rust
pub trait MyTrait {
    // What to return here?
    // This applies to the `Default` trait too
    fn new_value() -> Self where Self: Sized;

    fn static_method() -> &'static str;
}
