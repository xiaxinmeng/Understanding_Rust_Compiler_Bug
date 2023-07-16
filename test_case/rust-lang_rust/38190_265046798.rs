
seas:mod-dir-regr alex$ cargo +beta build
   Compiling mod-dir-regr v0.1.0 (file:///Users/alex/mod-dir-regr)
error: file not found for module `wrapper`
 --> src/lib.rs:8:9
  |
8 |     mod wrapper;
  |         ^^^^^^^
  |
  = help: name the file either wrapper.rs or wrapper/mod.rs inside the directory ""

error: Could not compile `mod-dir-regr`.

To learn more, run the command again with --verbose.
