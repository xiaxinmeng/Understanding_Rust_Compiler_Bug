 rust
fn open_custom_file() -> File {
    unsafe {
        let my_fd = libc::open(...);
        File::from_raw_fd(my_fd)
    }
}
fn my_c_api_wrapper() -> File {
    unsafe {
        let fd = my_c_api::something_that_returns_an_fd();
        File::from_raw_fd(fd)
    }
}
