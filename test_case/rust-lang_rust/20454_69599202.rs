 rust
    for i in range(0, 4) {
        let my_numbers = shared_numbers.clone();
        let my_frame_counter = shared_frame_counter.clone();
        Thread::spawn(move || -> () {
            loop {
                my_numbers[i] = (my_frame_counter.frame() / i as i32) % 240;
                t::delay(16);
            }
            return ();
        });
    }
