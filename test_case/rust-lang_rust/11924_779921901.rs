rust
struct InlineArray<T> {
    align: [T; 0],
    ... // inline or out-of-line storage
}

enum Recursive {
    A(InlineArray<Recursive>),
    B,
}
