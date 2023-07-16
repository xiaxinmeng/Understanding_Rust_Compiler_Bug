
// Macro definition
macro_rules! foo {
    ($($id:ident),*) => {}
}

// Invocation
let w;
let x;
let y;
let z;
foo!(w, x, y, z);
