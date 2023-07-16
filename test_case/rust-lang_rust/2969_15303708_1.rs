 rust
fn foo11() { // owned vector, mutable borrow
    let x = ~[1, 2, 4];
    let v : &mut int = &mut x[2];
    x[2] = 6; // error: assigning to mutable vec content prohibited due to outstanding loan
    fail_unless!( *v == 6 );
}
