Rust
// in mod A

macro foo($expand: ident) {
    $expand!(println!("foo"));
}

// in mod B

macro expand($macro:ident ! $args:tt) {
    $macro ! $args
}
foo!(expand); // this expands `println!("foo")` here.
