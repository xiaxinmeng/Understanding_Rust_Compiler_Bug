rust
struct Bug {
    A: [(); {
        let x: usize;
        x
    }],
}
