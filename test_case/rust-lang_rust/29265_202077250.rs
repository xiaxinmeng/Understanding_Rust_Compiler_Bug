
$ cargo build --verbose

   Compiling test_1_lib v0.1.0 (file:///private/tmp/rust-bug-1/bin)
     Running `rustc /private/tmp/rust-bug-1/lib/src/lib.rs --crate-name test_1_lib --crate-type lib -g -C metadata=50d0c76dcbac1234 -C extra-filename=-50d0c76dcbac1234 --out-dir /private/tmp/rust-bug-1/bin/target/debug/deps --emit=dep-info,link -L dependency=/private/tmp/rust-bug-1/bin/target/debug/deps -L dependency=/private/tmp/rust-bug-1/bin/target/debug/deps`
   Compiling test_1_bin v0.1.0 (file:///private/tmp/rust-bug-1/bin)
     Running `rustc src/test_1_bin.rs --crate-name test_1_bin --crate-type bin -g --out-dir /private/tmp/rust-bug-1/bin/target/debug --emit=dep-info,link -L dependency=/private/tmp/rust-bug-1/bin/target/debug -L dependency=/private/tmp/rust-bug-1/bin/target/debug/deps --extern test_1_lib=/private/tmp/rust-bug-1/bin/target/debug/deps/libtest_1_lib-50d0c76dcbac1234.rlib`
Could not compile `test_1_bin`.

Caused by:
  Process didn't exit successfully: `rustc src/test_1_bin.rs --crate-name test_1_bin --crate-type bin -g --out-dir /private/tmp/rust-bug-1/bin/target/debug --emit=dep-info,link -L dependency=/private/tmp/rust-bug-1/bin/target/debug -L dependency=/private/tmp/rust-bug-1/bin/target/debug/deps --extern test_1_lib=/private/tmp/rust-bug-1/bin/target/debug/deps/libtest_1_lib-50d0c76dcbac1234.rlib` (signal: 11)
