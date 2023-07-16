rust
macro m($i:ident) {
    pub struct $i; // (1)
    $i; // resolves to (1)
    A; //~ ERROR A is not in scope
}

fn f() {
    m!(A);
    A; // resolves to (1)
}
