
$ multirust run nightly cargo build --verbose

multirust: checking metadata version
multirust: got metadata version 2
   Compiling test_1_lib v0.1.0 (file:///private/tmp/rust-bug-1/lib)
     Running `rustc /private/tmp/rust-bug-1/lib/src/lib.rs --crate-name test_1_lib --crate-type lib -g -C metadata=fc60932e754a9e26 -C extra-filename=-fc60932e754a9e26 --out-dir /private/tmp/rust-bug-1/bin/target/debug/deps --emit=dep-info,link -L dependency=/private/tmp/rust-bug-1/bin/target/debug/deps -L dependency=/private/tmp/rust-bug-1/bin/target/debug/deps`
   Compiling test_1_bin v0.1.0 (file:///private/tmp/rust-bug-1/bin)
     Running `rustc src/test_1_bin.rs --crate-name test_1_bin --crate-type bin -g --out-dir /private/tmp/rust-bug-1/bin/target/debug --emit=dep-info,link -L dependency=/private/tmp/rust-bug-1/bin/target/debug -L dependency=/private/tmp/rust-bug-1/bin/target/debug/deps --extern test_1_lib=/private/tmp/rust-bug-1/bin/target/debug/deps/libtest_1_lib-fc60932e754a9e26.rlib`
