rust
#[macro_attr]
struct S {
    field: mac!() // also can expand this first only if we are using eager expansion
}
