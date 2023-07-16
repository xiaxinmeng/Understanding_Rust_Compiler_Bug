Rust
fn foo3c(n: u64) -> u64 {
    let mut count = 0;
    (0..n).for_each(|_| {
        (0..n).chain(::std::iter::once(n)).rev().for_each(|j| {
            count += j;
        })
    });
    count
}
