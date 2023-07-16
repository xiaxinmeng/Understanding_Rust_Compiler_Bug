 shell
fauna:beep edwinamsler$ cargo build --verbose
       Fresh num-traits v0.1.35
       Fresh quasi v0.16.0
       Fresh serde v0.7.15
       Fresh aster v0.22.1
       Fresh itoa v0.1.1
       Fresh log v0.3.6
       Fresh serde_codegen_internals v0.4.0
       Fresh quasi_codegen v0.16.0
       Fresh serde_json v0.7.4
       Fresh quasi_macros v0.16.0
       Fresh serde_codegen v0.7.15
       Fresh serde_macros v0.7.15
   Compiling beep v0.1.0 (file:///private/tmp/beep)
     Running `rustc src/main.rs --crate-name beep --crate-type bin -g -C metadata=a96ef1cdca33b93b --out-dir /private/tmp/beep/target/debug --emit=dep-info,link -L dependency=/private/tmp/beep/target/debug/deps --extern serde_json=/private/tmp/beep/target/debug/deps/libserde_json-55921106e25a0359.rlib --extern serde=/private/tmp/beep/target/debug/deps/libserde-036bf26779b9b9ba.rlib --extern serde_macros=/private/tmp/beep/target/debug/deps/libserde_macros-31e9066befad2eb9.dylib --extern log=/private/tmp/beep/target/debug/deps/liblog-bf16bb9a4912b11d.rlib`
src/main.rs:8:5: 8:9 error: macro undefined: 'info!' 
src/main.rs:8     info!("Hi!");
                  ^~~~
error: aborting due to previous error 
error: Could not compile `beep`.

Caused by:
  Process didn't exit successfully: `rustic src/main.rs --crate-name beep --crate-type bin -g -C metadata=a96ef1cdca33b93b --out-dir /private/tmp/beep/target/debug --emit=dep-info,link -L dependency=/private/tmp/beep/target/debug/deps --extern serde_json=/private/tmp/beep/target/debug/deps/libserde_json-55921106e25a0359.rlib --extern serde=/private/tmp/beep/target/debug/deps/libserde-036bf26779b9b9ba.rlib --extern serde_macros=/private/tmp/beep/target/debug/deps/libserde_macros-31e9066befad2eb9.dylib --extern log=/private/tmp/beep/target/debug/deps/liblog-bf16bb9a4912b11d.rlib` (exit code: 101)
