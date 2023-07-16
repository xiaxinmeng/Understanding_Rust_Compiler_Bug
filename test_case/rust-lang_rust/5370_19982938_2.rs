
% cat ~/Dev/Rust/iss5370-b.rs
pub struct ReaderError<'self>(&'self mut int);

condition! {
    pub reader_error: super::ReaderError -> bool;
}

fn foo<'b>(err: ReaderError<'b>) -> bool {
    reader_error::cond.raise(err)
}

fn main() {
    let mut x : int = 2;
    let mut y : int = 3;
    let err = ReaderError(&x);
    do reader_error::cond.trap(|z| {
        println(fmt!("Hi from handler I think z: %?", z));
        true
    }).in {
        foo(err, &mut y);
    }
}
% rustc ~/Dev/Rust/iss5370-b.rs
/Users/fklock/Dev/Rust/iss5370-b.rs:8:29: 8:32 error: internal compiler error: Cannot relate bound region as subregion: br_anon(0)
/Users/fklock/Dev/Rust/iss5370-b.rs:8     reader_error::cond.raise(err)
                                                                   ^~~
% 
