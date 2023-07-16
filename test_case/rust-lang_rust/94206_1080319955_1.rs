
    let value = mutex1.lock().unwrap().s.len() < mutex2.lock().unwrap().s.len();
    match value {
        true => {
            println!("{} < {}", mutex1.lock().unwrap().s.len(), mutex2.lock().unwrap().s.len());
        }
        false => {}
    };
