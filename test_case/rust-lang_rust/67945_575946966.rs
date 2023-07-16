rust
struct Bug<S:?Sized> {
    A: [(); {
        let x: Option<Box<Self>> = None;
        0
    }],
    B: S
}
