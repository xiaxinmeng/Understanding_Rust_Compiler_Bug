rust
    let mut ff_loop_abort;
    loop {  // Resize loop
        ff_loop_abort = false;
        ... lots of code here ...
    }
    if ff_loop_abort {
        println!("loop aborted abnormally");
    }
