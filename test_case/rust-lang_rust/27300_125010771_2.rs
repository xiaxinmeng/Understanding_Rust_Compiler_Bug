 rust
let p = Some(45).and_then({|x|
    println!("doubling {}", x);
    Some(x * 2)
})
