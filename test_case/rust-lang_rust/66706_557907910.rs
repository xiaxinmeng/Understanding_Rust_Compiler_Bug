Rust
fn bug() {
    [0; [|&_: _ &_| {}; 0 ].len()]
}
