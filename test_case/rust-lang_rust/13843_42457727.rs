
#![feature(phase)]
extern crate regex;
#[phase(syntax)] extern crate regex_macros;

fn main() {
    // error: internal compiler error: unexpected failure
    // note: the compiler hit an unexpected failure path. this is a bug.
    // note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
    // note: run with `RUST_BACKTRACE=1` for a backtrace
    // task 'rustc' failed at 'index out of bounds: the len is 0 but the index is 0', /build/rust-git/src/rust/src/libregex/lib.rs:1
    let compile_time_empty_re = regex!("");

    // comment out the regex! macro to get the task failure on runtime for the following:
    let run_time_empty_re = regex::Regex::new("").unwrap();
}
