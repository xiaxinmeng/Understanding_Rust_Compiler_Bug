
    unsafe {
        let x5: *const Data = std::mem::transmute(a5.bar);
        println!("{:}", (*x5).bar);
    }
