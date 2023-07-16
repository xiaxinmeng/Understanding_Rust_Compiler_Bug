
  Compiling playground v0.0.1 (file:///playground)
warning: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
 --> src/main.rs:5:1
  |
5 | / fn foo() {
6 | | }
  | |_^
  |
note: lint level defined here
 --> src/main.rs:2:9
  |
2 | #![warn(absolute_paths_not_starting_with_crate)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
  = note: for more information, see issue TBD
help: use `crate`
  |
5 | crate::fn foo() {
6 | }
  |

    Finished dev [unoptimized + debuginfo] target(s) in 1.93s
     Running target/debug/deps/playground-59295fed136eb70e
