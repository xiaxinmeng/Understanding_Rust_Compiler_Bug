 rust
use std::env;
use std::ffi::OsString;

fn foo<F>(bar: &F) -> Option<OsString>
    where F: for <'r> Fn(&'r str) -> Option<OsString>
{
    bar("HOME")
}

fn main() {
    println!("{:?}", foo(&|x|env::var_os(x)));
}
