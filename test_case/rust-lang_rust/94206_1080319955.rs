
    match mutex1.lock().unwrap().s.len() < mutex2.lock().unwrap().s.len() {
        true => {
            println!("{} < {}", mutex1.lock().unwrap().s.len(), mutex2.lock().unwrap().s.len());
        }
        false => {}
    };
