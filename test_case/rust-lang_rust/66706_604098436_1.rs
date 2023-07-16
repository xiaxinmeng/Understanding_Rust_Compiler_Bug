rust
fn bug() {
    [0; [|_: _ &_| ()].len()]
}
