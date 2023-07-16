
    let x = 0;
    let mut y = move |z: u32| {
        let static mut x = x + z;
        println!("inner x {}", x);
    };
