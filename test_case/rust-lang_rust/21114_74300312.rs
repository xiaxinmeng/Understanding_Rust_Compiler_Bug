
    std::old_io::stdio::stdin().lock().lines().map( |line| {
        line.unwrap().trim().to_string()
    }).collect()
