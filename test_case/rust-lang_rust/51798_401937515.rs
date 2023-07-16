
+ /home/jeremy/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type lib --out-dir ./target -L dependency=./target child.rs --crate-name rust_issue_51798_example_child -C metadata=arbitrary_child_id -C extra-filename=-arbitrary_child_id
+ /home/jeremy/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type lib --out-dir ./target -L dependency=./target parent.rs --crate-name rust_issue_51798_example_parent -C metadata=arbitrary_parent_id -C extra-filename=-arbitrary_parent_id --extern rust_issue_51798_example_child=./target/librust_issue_51798_example_child-arbitrary_child_id.rlib
error: internal compiler error: no type-dependent def for method call
 --> parent.rs:8:9
  |
8 |         v.clear();
  |         ^^^^^^^^^

error: aborting due to previous error
