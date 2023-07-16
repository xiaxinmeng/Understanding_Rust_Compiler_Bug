rust
#![warn(noop_method_call)]

struct PlainType<T>(T);

pub fn foo(non_clone_type_ref: &PlainType<u32>) -> PlainType<u32> {
    non_clone_type_ref.clone()
}
