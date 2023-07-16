
       Fresh libc v0.1.10
       Fresh regex-syntax v0.2.2
       Fresh memchr v0.1.6
       Fresh aho-corasick v0.3.2
       Fresh regex v0.1.41
   Compiling bad-rust-plugin v0.1.0 (file:///Users/lazarus/dev/tmp/bad-rust-plugin)
     Running `./x86_64-apple-darwin/stage2/bin/rustc /Users/lazarus/dev/tmp/bad-rust-plugin/src/main.rs --crate-name bad_rust_plugin --crate-type bin -g --out-dir /Users/lazarus/dev/tmp/bad-rust-plugin/target/debug --emit=dep-info,link -L dependency=/Users/lazarus/dev/tmp/bad-rust-plugin/target/debug -L dependency=/Users/lazarus/dev/tmp/bad-rust-plugin/target/debug/deps --extern regex_macros=/Users/lazarus/dev/tmp/bad-rust-plugin/target/debug/deps/libregex_macros-ed47cfffdeca22b7.dylib --extern regex=/Users/lazarus/dev/tmp/bad-rust-plugin/target/debug/deps/libregex-c5abedf84fe61ddb.rlib`
       Fresh regex_macros v0.1.21
/Users/lazarus/dev/tmp/bad-rust-plugin/src/main.rs:6:26: 6:31 error: macro undefined: 'regex!'
/Users/lazarus/dev/tmp/bad-rust-plugin/src/main.rs:6 static R: regex::Regex = regex!("boom");
                                                                              ^~~~~
error: aborting due to previous error
Could not compile `bad-rust-plugin`.

Caused by:
  Process didn't exit successfully: `./x86_64-apple-darwin/stage2/bin/rustc /Users/lazarus/dev/tmp/bad-rust-plugin/src/main.rs --crate-name bad_rust_plugin --crate-type bin -g --out-dir /Users/lazarus/dev/tmp/bad-rust-plugin/target/debug --emit=dep-info,link -L dependency=/Users/lazarus/dev/tmp/bad-rust-plugin/target/debug -L dependency=/Users/lazarus/dev/tmp/bad-rust-plugin/target/debug/deps --extern regex_macros=/Users/lazarus/dev/tmp/bad-rust-plugin/target/debug/deps/libregex_macros-ed47cfffdeca22b7.dylib --extern regex=/Users/lazarus/dev/tmp/bad-rust-plugin/target/debug/deps/libregex-c5abedf84fe61ddb.rlib` (exit code: 101)
