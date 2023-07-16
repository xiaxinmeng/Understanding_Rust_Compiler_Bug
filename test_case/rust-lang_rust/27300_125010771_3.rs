 rust
let p = Some(45).and_then({
    |x| { println!("doubling {}", x) }; // braces optional
    Some(x * 2) // returns `Some(i32)`
})
