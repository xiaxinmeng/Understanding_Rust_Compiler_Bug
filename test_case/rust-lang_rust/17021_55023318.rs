
$ rustc --crate-type lib -g b17021.rs 
b17021.rs:3:1: 5:2 warning: code is never used: `bug`, #[warn(dead_code)] on by default
b17021.rs:3 fn bug() {
b17021.rs:4   let c = box |&:| {};
b17021.rs:5 }
b17021.rs:4:7: 4:8 warning: unused variable: `c`, #[warn(unused_variable)] on by default
b17021.rs:4   let c = box |&:| {};
                  ^
error: internal compiler error: get_unique_type_id_of_type() - unexpected type: closure, ty_unboxed_closure(syntax::ast::DefId{krate: 0u32, node: 12u32}, ReScope(6u32))
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ast_util.rs:776
