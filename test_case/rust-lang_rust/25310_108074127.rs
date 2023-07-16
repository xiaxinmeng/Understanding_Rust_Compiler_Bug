 rust
use std::env;
use std::ffi::{OsStr, OsString};

fn test<K:?Sized>(_: &K) -> Option<OsString> where K: AsRef<OsStr> { None }

/* Explicit lifetimes are required by rust-2015-05-11 nightly (newer than playpen) */
fn foo<'a, F>(bar: &'a F) -> Option<OsString>
    where F: Fn(&str) -> Option<OsString>
{
    bar("HOME")
}

fn main() {
    println!("{:?}", foo(&test));
}
