rust
fn foo(x: NonCopyStruct) {
    let y = x;
    let NonCopyStruct { copy_field: z } = x;
}
