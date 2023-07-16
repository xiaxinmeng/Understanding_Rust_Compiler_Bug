`rust
    if yes {
       let mut x = Vec::new();
	x.push(1);
        x.push(2);
        x.push(3);
        x.push(4);
    } else {
        let mut x = Vec::new();
        x.push(1);
        x.push(2);
        x.push(3);
        x.push(4);
        x.push(5);
        x.push(6);
    }
    println!("vec: {:?}", x);
