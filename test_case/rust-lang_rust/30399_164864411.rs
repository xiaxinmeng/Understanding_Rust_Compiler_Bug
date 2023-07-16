 rust
    let v = vec![255u8; 5000];
    let s = String::from_utf8_lossy(&v);
    // println!("Len {}", s.len());
    println!("{}", s);
