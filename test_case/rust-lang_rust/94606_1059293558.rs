plain
[TIMING] Rustc { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-illumos", file: None } } } -- 22.032
Dist rust-std-nightly-x86_64-unknown-illumos
 finished in 10.211 seconds
[TIMING] Std { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-illumos", file: None } } -- 10.213
thread 'main' panicked at 'could not read dir "/checkout/src/build_helper": Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/bootstrap/lib.rs:1462:25
Build completed unsuccessfully in 0:14:04
