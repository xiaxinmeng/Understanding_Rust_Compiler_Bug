
foo.rs:8:23: 8:34 error: internal compiler error: find_associated_type_in_generics(): didn't find associated type anywhere in the generics list
foo.rs:8     fn deserialize<D: Deserialize>();
                               ^~~~~~~~~~~
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:113
