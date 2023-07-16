
% cat ~/Dev/Rust/iss5370-a.rs
pub struct ReaderError<'self>(&'self mut int);

condition! {
    pub reader_error: super::ReaderError -> bool;
}

fn main() {
}
% rustc --version
/Users/fklock/opt/rust/bin/rustc 0.6 (e650399 2013-06-12 23:10:15 -0700)
host: x86_64-apple-darwin
% rustc ~/Dev/Rust/iss5370-a.rs
warning: no debug symbols in executable (-arch x86_64)
% ~/Dev/Rust/iss5370-a
% 
