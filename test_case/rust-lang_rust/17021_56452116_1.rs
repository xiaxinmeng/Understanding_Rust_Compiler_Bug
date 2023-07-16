
$ cargo test --verbose
   Compiling replace-map v0.0.1 (file:///home/foo/rust/rust-replace-map)
     Running `rustc src/lib.rs --crate-name replace-map --crate-type lib -g --test -C metadata=2317281ca46f881b -C extra-filename=-2317281ca46f881b --out-dir /home/foo/rust/rust-replace-map/target --dep-info /home/foo/rust/rust-replace-map/target/.fingerprint/replace-map-1515ca252146df0a/dep-test-lib-replace-map -L /home/foo/rust/rust-replace-map/target -L /home/foo/rust/rust-replace-map/target/deps`
error: internal compiler error: get_unique_type_id_of_type() - unexpected type: closure, ty_unboxed_closure(syntax::ast::DefId{krate: 0u32, node: 67u32}, ReScope(64u32))
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:169


Could not compile `replace-map`.
