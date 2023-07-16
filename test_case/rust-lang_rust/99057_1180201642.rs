diff
 $ cargo b --verbose
        Fresh unicode-ident v1.0.1
    Compiling example v0.1.0 (/projects/example)
    Compiling ...
    Compiling serde v1.0.139
+      Running `rustc --crate-name build_script_build --edition=2021 build.rs --error-format=json
+      --json=diagnostic-rendered-ansi,artifacts,fu ture-incompat --crate-type bin
+      --emit=dep-info,link -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2
+      -C metadata=39a8aabaeae1d ec5 -C extra-filename=-39a8aabaeae1dec5 --out-dir
+      /projects/example/target/debug/build/example-39a8aabaeae1dec5 -C
+      incremental=/projects/example/target/debug/incremental
+      -L dependency=/projects/example/target/debug/deps`
      Running ...
      Running ...
 error[E0433]: failed to resolve: use of undeclared crate or module `serde`
  --> src/sometimes_working/mod.rs:1:30
   |
 1 | #[derive(Clone, Debug, Hash, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
   |                              ^^^^^ use of undeclared crate or module `serde`
 
 error[E0433]: failed to resolve: use of undeclared crate or module `serde`
  --> src/sometimes_working/mod.rs:1:50
   |
 1 | #[derive(Clone, Debug, Hash, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
   |                                                  ^^^^^ use of undeclared crate or module `serde`
 
 For more information about this error, try `rustc --explain E0433`.
 error: could not compile `example` due to 2 previous errors
 
+ Caused by:
+   process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021
+   build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat
+   --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C split-debuginfo=unpacked
+   -C debuginfo=2 -C metadata=39a8aabaeae1dec5 -C extra-filename=-39a8aabaeae1dec5
+   --out-dir /projects/example/target/debug/build/example-39a8aabaeae1dec5
+   -C incremental=/projects/example/target/debug/incremental
+   -L dependency=/projects/example/target/debug/deps` (exit status: 1)
 warning: build failed, waiting for other jobs to finish...

