
    arr.into_iter().map(move |_e| {
        async move {
            let value = cloned_value.clone();
            println!("{:?}", value.lock().ok())
        }
    })
