rust
#[only(copy)]
struct Example {
    field: String, // error because not copy
}
