 rust
fn foo21() { // heap-mutable vector, mutable borrow
    let x = @mut [1, 2, 4];
    let v : &mut int = &mut x[2];
    x[2] = 6;
    fail_unless!( *v == 6 );
}
