rust
fn main() {
    let x = This::that(&[]);
}

// equivalent to
fn main() {
    // [...] literals are always array types, not slices
    let array: [?0; 0] = [];

    // Each type parameter in a generic function call gets a fresh
    // type inference variable, including the Self type. (?1 below)
    let arg: &?1 = &array;
    let func = <?1 as This>::that;
    let x = func(arg);
}
