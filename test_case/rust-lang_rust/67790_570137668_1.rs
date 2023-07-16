rust
fn main() {
    let annotated: &[_] = &[];
    let x = This::that(annotated);
}

// equivalent to
fn main() {
    let array: [?0; 0] = [];
    let annotated: &[?2] = &array;

    let arg: &?1 = annotated;
    let func = <?1 as This>::that;
    let x = func(arg);
}
