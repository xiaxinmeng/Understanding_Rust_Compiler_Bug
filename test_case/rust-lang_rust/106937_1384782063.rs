rust
fn main() {
    // Will print "Uncategorized".
    print_error(Error::last_os_error());
