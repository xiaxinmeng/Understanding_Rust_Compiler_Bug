rust
#[derive(Derivative)]
#[derivative(Debug)]
struct MyStruct<T> {
    #[derivative(Debug(bound=""))] // or "typ!(): ::std::fmt::Debug"
    field: typ!(),
}
