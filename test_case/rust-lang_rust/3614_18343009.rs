 rust
struct S;

#[doc = "this works"]
/// this too
impl S {
    #[doc = "this doesn't work"];
    //! not this either
}
