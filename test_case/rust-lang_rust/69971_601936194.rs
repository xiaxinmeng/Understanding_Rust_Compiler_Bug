rust
fn foo(x: NonCopyStruct) {
    let NonCopyStruct { copy_field: z } = x;
    let y = x;
}
