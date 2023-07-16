
error: unsupported operation: unsupported Linux dlsym: getrandom
   --> /home/mara/dev-ext/rust/library/std/src/sys/unix/weak.rs:100:5
    |
100 |     libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr()) as usize
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsupported Linux dlsym: getrandom
