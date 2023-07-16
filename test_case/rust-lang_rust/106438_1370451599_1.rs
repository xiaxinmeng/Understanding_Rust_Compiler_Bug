
➜  poc git:(master) ✗ rustc src/lib.rs --crate-type=lib
error: captured variable cannot escape `FnMut` closure body
  --> src/lib.rs:9:17
   |
8  |   fn foo(f: &mut dyn Fn()) {
   |          - variable defined here
9  |       run(move || Box::new(|| {
   |  _______________-_^
   | |               |
   | |               inferred to be a `FnMut` closure
10 | |         // dummy lines to push `f` off the screen
11 | |         //
12 | |         //
13 | |         f();
   | |         - variable captured here
14 | |         //
15 | |         //
16 | |     }));
