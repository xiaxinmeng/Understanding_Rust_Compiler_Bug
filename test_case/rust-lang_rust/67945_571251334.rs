rust
enum Bug<S> {
    Var = {
        let x: S = 0;
        0
    },
}
