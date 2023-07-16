
> let data = Arc::new(Mutex::new(Vec::new()));
> let err = Sink(data.clone());
> 
> let r = thread::spawn(|| {
>     io::set_panic(Box::new(err));
>     panic!();
> }).join();
> println!("{:?}", r.is_err());
> 