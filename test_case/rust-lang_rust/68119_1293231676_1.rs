rust
    arr.into_iter().map(move |_e| {
        let value = cloned_value.clone();
        async move {
            println!("{:?}", value.lock().ok())
        }
    })
