 rust
fn foo20() { // heap-mutable vector, immutable borrow
    let x = @mut [1, 2, 4];
    let v : &int = &x[2]; // error: illegal borrow unless pure: creating immutable alias to mutable vec content
    x[2] = 6; // note: impure due to assigning to mutable vec content
    fail_unless!( *v == 6 );
}
