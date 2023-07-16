rust
macro_rules! maybe_return { ($e:expr) => ($e); }

fn frob(x: i32) -> i32{
    maybe_return! {x}
    // should return -1 
    -1
}
