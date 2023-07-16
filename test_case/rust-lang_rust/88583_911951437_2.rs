rust
struct union { field: u32 };
struct S { field: union };
fn main() {
    let s = S { field: union { field: 42 } };
}
