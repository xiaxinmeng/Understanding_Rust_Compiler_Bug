
<anon>:5:23: 5:31 error: internal compiler error: find_associated_type_in_generics(): didn't find associated type anywhere in the generics list
<anon>:5 trait Foo<A> where A: Bindable {
                               ^~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /build/rust-git/src/rust/src/libsyntax/diagnostic.rs:116
