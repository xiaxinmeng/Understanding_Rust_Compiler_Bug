rust
fn bar<const N: usize>() where (if N > 0 {
        25 / N
    } else {
        0
    }) {
    let _: [u8; if N > 0 {
        25 / N
    } else {
        0
    }]
}
