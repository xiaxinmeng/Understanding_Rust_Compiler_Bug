 rust
macro_rules! foo {
    ($e: expr) => {
        // $e shouldn't be able to interact with this 'x
        'x: loop { $e }
    }
}

'x: loop {
    // i.e. this 'x should refer to the outer loop, but may be "captured"
    // by the 'x declaration inside foo
    foo!(break 'x);
    println!("should not be printed")
}
