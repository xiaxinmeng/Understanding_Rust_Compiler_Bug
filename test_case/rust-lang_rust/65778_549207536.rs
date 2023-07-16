
    let a = Rc::new(1);
    let c = Rc::downgrade(&a);
    drop(a);
    let d = c.clone();
    println!("{:?}", d.weak_count());
