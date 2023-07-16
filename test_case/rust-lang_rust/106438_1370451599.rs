
➜  poc git:(master) ✗ cargo check
    Checking poc v0.1.0 (/tmp/poc)
error: captured variable cannot escape `FnMut` closure body
  --> src/lib.rs:9:17
   |
9  |       run(move || Box::new(|| {
   |  _______________-_^
   | |               |
   | |               inferred to be a `FnMut` closure
10 | |         // dummy lines to push `f` off the screen
11 | |         //
12 | |         //
...  |
15 | |         //
16 | |     }));
