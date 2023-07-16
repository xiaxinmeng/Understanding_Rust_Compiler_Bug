
$ cat src/main.rs 
fn main() {
    /// crash
    println!("{}", "Hello, world!");
}
$ cargo build
   Compiling example_project v0.1.0 (file:///home/ubuntu/example_project)
error: attributes on non-item statements and expressions are experimental. (see issue #15701)
 --> src/main.rs:2:5
  |
2 |     /// crash
  |     ^^^^^^^^^

error: aborting due to previous error

error: Could not compile `example_project`.

To learn more, run the command again with --verbose.
$ cargo doc
 Documenting example_project v0.1.0 (file:///home/ubuntu/example_project)
error: attributes on non-item statements and expressions are experimental. (see issue #15701)
 --> src/main.rs:2:5
  |
2 |     /// crash
  |     ^^^^^^^^^

error: Compilation failed, aborting rustdoc

error: Could not document `example_project`.

To learn more, run the command again with --verbose.
$
