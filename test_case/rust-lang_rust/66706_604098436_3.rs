rust
fn bug() {
    [0; match [|f @ &ref _| () ] {} ]
}
