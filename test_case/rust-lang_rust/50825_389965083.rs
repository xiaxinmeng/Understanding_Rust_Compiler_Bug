rust
fn foo(y: &[()]) where u64: Copy // or From<u64>
{
    let x = y[0]; // u64: std::slice::SliceIndex<[()]>` is not satisfied
}
