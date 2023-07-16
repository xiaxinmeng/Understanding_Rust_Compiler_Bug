rust
fn main() {
    [(); {
        let a = 10_usize;
        let b: &'_ usize = &a;
        *b
    }]
}
