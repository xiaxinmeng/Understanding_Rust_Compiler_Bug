rust
fn foo() {
    let list = vec![Some(1), None, Some(3)];
    
    for reference in list {
        if /*let*/ Some(reference) = reference {
            unimplemented!()
        }
    }
}
