rust
fn f() {
    do_stuff();
    #[cfg(unix)]
    {
        do_unix_stuff();
    }
    // or even
    #[cfg(unix)]
    do_unix_stuff();
}
