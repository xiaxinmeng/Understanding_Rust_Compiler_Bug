rust
let fd = object.as_raw_fd();

take_fd(fd);

// Probably going to get an `io::Error` here
object.do_something_else();
