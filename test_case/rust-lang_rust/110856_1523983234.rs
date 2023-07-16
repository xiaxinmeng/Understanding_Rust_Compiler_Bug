rust
    let mut x = String::from("hello");
    let z = &x;
    let y = &mut x;
    println!("{y}{z}");
