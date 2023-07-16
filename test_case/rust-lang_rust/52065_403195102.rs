rust
    (0..10)
        .inspect(|x| print!("({}) ", x))
        .step_by(2)
        .take(3)
        .for_each(|x| println!("{}", x));
