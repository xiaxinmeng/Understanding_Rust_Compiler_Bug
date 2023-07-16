 rust
fn foo(i: uint) -> uint {
    enforce!(i < 10, "The Algorithm only works for i values 0-9");
    /* complicated algorithm */
    assert!(result <= 1024); // This should always be true if the algorithm is correct
    result
}
