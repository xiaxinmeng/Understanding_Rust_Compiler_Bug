
thread 'rustc' panicked at 'type mismatch when copying!
src: for<'r, 's> fn(u32, &'r str, &'s std::cell::RefCell<std::vec::Vec<u32>>),
dest: for<'r> fn(u32, &'r str, &std::cell::RefCell<std::vec::Vec<u32>>)', src/librustc_mir/interpret/place.rs:883:9
