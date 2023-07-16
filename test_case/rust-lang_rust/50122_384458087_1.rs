Rust
// in mod A

macro foo($bang: tt) {
    println $bang ("foo")
}

// in mod B

foo!(!); // this expands `println!("foo")` here.
