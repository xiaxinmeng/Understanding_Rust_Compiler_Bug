rust
    let foo = Mutex::new("123");

    let foo: Box<dyn futures::future::Future<Output = ()> + Send> = Box::new(async move {
        // In a separate block works
        // {
        let bar = foo.lock().unwrap();
        drop(bar);
        // }
        futures::future::lazy(|_| ()).await;
    });
