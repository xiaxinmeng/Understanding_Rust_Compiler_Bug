rust
struct MyStruct {
    my_field: i32,
}

fn main() {
    // Expands to quote!($input { my_field: 0 }).
    // Currently requires `my_field: 0` to be spanned call site.
    // Should work without that.
    pp!(MyStruct);
}
