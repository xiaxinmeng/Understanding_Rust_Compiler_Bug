
panic::set_hook(Box::new(|panic_info| {
    println!("panic occurred: {:?}", panic_info.payload().downcast_ref::<&str>().unwrap());
}));

> thread panicked while processing panic. aborting.   
> Illegal instruction (core dumped)
