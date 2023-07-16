rust
fn cond(i: usize) -> bool { i % 2 == 0 }
pub struct SomeData(usize);
pub fn foo() -> Vec<SomeData> {
    const N: usize = 1_000_000;
    let mut data = Vec::with_capacity(N);

    for i in 0 .. N {
        if cond(i) {
            data.push(SomeData(i));
        }
    }
    data
}
