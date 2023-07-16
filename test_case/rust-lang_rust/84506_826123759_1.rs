rust
    thread.max_7456.add_exec(max7456.run());
    let int = make_trigger(thread.max_7456, periph_exti3!(reg));
    ...
    thread.bmp_280.add_fn(never_complete(move || bmp280.trigger(&rx, &tx)));
    let int = make_trigger(thread.bmp_280, periph_exti2!(reg));
