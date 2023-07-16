
% cat ~/Dev/Rust/iss5370-c.rs
pub struct ReaderError<'self>(&'self mut int);

condition! {
    pub reader_error: super::ReaderError -> bool;
}

fn main() {
    let mut x : int = 1;
    do reader_error::cond.trap(|z| {
        println(fmt!("Hi from handler I think z: %?", z));
        true
    }).in {
        println("Start of try");

        reader_error::cond.raise(ReaderError(&mut x));

        println("Finis of try");
    }
}
% rustc ~/Dev/Rust/iss5370-c.rs
/Users/fklock/Dev/Rust/iss5370-c.rs:15:45: 15:51 error: borrowed value does not live long enough
/Users/fklock/Dev/Rust/iss5370-c.rs:15         reader_error::cond.raise(ReaderError(&mut x));
                                                                                    ^~~~~~
note: borrowed pointer must be valid for the static lifetime...
/Users/fklock/Dev/Rust/iss5370-c.rs:7:10: 19:1 note: ...but borrowed value is only valid for the block at 7:10
/Users/fklock/Dev/Rust/iss5370-c.rs:7 fn main() {
/Users/fklock/Dev/Rust/iss5370-c.rs:8     let mut x : int = 1;
/Users/fklock/Dev/Rust/iss5370-c.rs:9     do reader_error::cond.trap(|z| {
/Users/fklock/Dev/Rust/iss5370-c.rs:10         println(fmt!("Hi from handler I think z: %?", z));
/Users/fklock/Dev/Rust/iss5370-c.rs:11         true
/Users/fklock/Dev/Rust/iss5370-c.rs:12     }).in {
                                       ...
error: aborting due to previous error
% 
