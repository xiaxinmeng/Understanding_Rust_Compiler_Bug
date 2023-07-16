
zmd@ReflectiveCoherence:~/Code/rusty-machine$ cargo +nightly test
   Compiling rusty-machine v0.5.4 (file:///home/zmd/Code/rusty-machine)
error[E0463]: can't find crate for `rm`
 --> tests/learning/dbscan.rs:1:5
  |
1 | use rm::linalg::Matrix;
  |     ^^ can't find crate

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error[E0463]: can't find crate for `linalg`
   --> src/data/transforms/minmax.rs:195:9
    |
195 |     use linalg::Matrix;
    |         ^^^^^^ can't find crate

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: Could not compile `rusty-machine`.
warning: build failed, waiting for other jobs to finish...
error: Could not compile `rusty-machine`.

To learn more, run the command again with --verbose.
