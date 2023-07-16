 console
$ rustc --version --verbose
rustc 1.0.0-nightly (083b8a404 2015-04-05) (built 2015-04-05)
binary: rustc
commit-hash: 083b8a40413eb3dfec7430150b8895f6c8bbafca
commit-date: 2015-04-05
build-date: 2015-04-05
host: x86_64-unknown-linux-gnu
release: 1.0.0-nightly
$ cat foo.rs
#![crate_type = "rlib"]

enum E { E0 = 0, E1 = 1 }
const E0_U8: u8 = E::E0 as u8;
const E1_U8: u8 = E::E1 as u8;

pub fn go<T>() {
    match 0 {
        E0_U8 => (),
        E1_U8 => (),
        _ => (),
    }
}

$ cat bar.rs
extern crate foo;

fn main() {
    foo::go::<()>();
}

$ rustc foo.rs
$ rustc bar.rs -L .
foo.rs:4:19: 4:30 error: non-constant path in constant expr
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'compare_list_exprs: type mismatch', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/trans/_match.rs:238
