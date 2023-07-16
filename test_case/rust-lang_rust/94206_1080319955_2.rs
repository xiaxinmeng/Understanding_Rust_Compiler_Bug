
        match (true, mutex1.lock().unwrap().s.len(), true) {
            (_, 3, _) => {
                println!("started");
                mutex1.lock().unwrap().s.len();
                println!("done");
            }
            (_, _, _) => {}
        };
