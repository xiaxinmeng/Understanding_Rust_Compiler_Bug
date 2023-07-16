
extern crate libc;

...

    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
