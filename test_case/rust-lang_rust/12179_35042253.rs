 rust
macro_rules! foo {
    () => {
        // this should just break from this inner loop...
        'x: loop { break 'x; }
    }
}

'x: loop {
    // ...but it may refer to the outer 'x when used inside here
    foo!();
    println!("should be printed");
}
