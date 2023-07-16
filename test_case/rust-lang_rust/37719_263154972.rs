rust
macro_rules! noop {
    ($e:expr) => {$e}
}

fn main() {
    noop!({
        // ... imagine lots of code here ...
        try!(0); // Oops!
        // ... imagine lots of code here ...
    })
}
