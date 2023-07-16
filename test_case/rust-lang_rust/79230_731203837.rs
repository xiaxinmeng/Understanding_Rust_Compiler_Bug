rust
fn print_error_with_code(err: io::Error) {
    match err.raw_os_error() {
        Some(code) => println!("{} (os error {})", err, code),
        None => println!("{}", err),
    }
}