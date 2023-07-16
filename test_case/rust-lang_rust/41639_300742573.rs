
ishitatsuyuki@X1Yoga  ~/D/rls   master  cargo build --release        
   Compiling rls v0.1.0 (file:///home/ishitatsuyuki/Documents/rls)
error[E0523]: found two different crates with name `bitflags` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
  --> src/build.rs:12:1
   |
12 | extern crate rustc;
   | ^^^^^^^^^^^^^^^^^^^

error: Could not compile `rls`.

To learn more, run the command again with --verbose.
