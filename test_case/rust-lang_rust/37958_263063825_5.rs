
     Running `rustc src/lib.rs --crate-name cross_proc_macro_example --crate-type lib -g -C metadata=e2ae6c3617222432 -C extra-filename=-e2ae6c3617222432 --out-dir /home/emk/w/src/pr/cross_proc_macro_example/target/x86_64-unknown-linux-musl/debug/deps --emit=dep-info,link --target x86_64-unknown-linux-musl -L dependency=/home/emk/w/src/pr/cross_proc_macro_example/target/x86_64-unknown-linux-musl/debug/deps --extern serde=/home/emk/w/src/pr/cross_proc_macro_example/target/x86_64-unknown-linux-musl/debug/deps/libserde-05bbf4d3437164cb.rlib --extern serde_derive=/home/emk/w/src/pr/cross_proc_macro_example/target/debug/deps/libserde_derive-4c72d69e014dae87.so`
     Running `rustc examples/example.rs --crate-name example --crate-type bin -g -C metadata=e2ae6c3617222432 -C extra-filename=-e2ae6c3617222432 --out-dir /home/emk/w/src/pr/cross_proc_macro_example/target/x86_64-unknown-linux-musl/debug/examples --emit=dep-info,link --target x86_64-unknown-linux-musl -L dependency=/home/emk/w/src/pr/cross_proc_macro_example/target/x86_64-unknown-linux-musl/debug/deps --extern serde=/home/emk/w/src/pr/cross_proc_macro_example/target/x86_64-unknown-linux-musl/debug/deps/libserde-05bbf4d3437164cb.rlib --extern serde_derive=/home/emk/w/src/pr/cross_proc_macro_example/target/debug/deps/libserde_derive-4c72d69e014dae87.so --extern cross_proc_macro_example=/home/emk/w/src/pr/cross_proc_macro_example/target/x86_64-unknown-linux-musl/debug/deps/libcross_proc_macro_example-e2ae6c3617222432.rlib`
error[E0463]: can't find crate for `serde_derive` which `cross_proc_macro_example` depends on
 --> examples/example.rs:1:1
  |
1 | extern crate cross_proc_macro_example;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error

error: Could not compile `cross_proc_macro_example`.
