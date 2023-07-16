
$ TMPDIR=fake rustc - <<< 'fn main() {}'
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'needs a temp dir', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libcore/option.rs:246
