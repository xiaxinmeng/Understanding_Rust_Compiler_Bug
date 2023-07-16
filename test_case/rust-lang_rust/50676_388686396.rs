rust
#[derive(Debug)]
struct MyStruct<T: FromStr> {
    field: T
}
