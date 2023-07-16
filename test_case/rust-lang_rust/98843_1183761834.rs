plain
1   1         Updating [..]
2   2      Downloading crates ...
3   3       Downloaded common v1.0.0 [..]
4   4        Compiling common v1.0.0
5   5          Running `rustc --crate-name common [..]
6        -     Running `rustc --crate-name common [..]
7   6        Compiling pm v0.1.0 [..]
    7    +     Running `rustc --crate-name common /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t767/home/.cargo/registry/src/-4dba4148552fc90b/common-1.0.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C metadata=3727013b635cbf1c -C extra-filename=-3727013b635cbf1c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t767/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t767/foo/target/debug/deps --cap-lints allow`
8   8          Running `rustc --crate-name pm [..]
9   9        Compiling foo v0.1.0 [..]
10  10         Running `rustc --crate-name foo [..]
11  11        Finished [..]

other output:

', src/tools/cargo/tests/testsuite/collisions.rs:372:10
---
test result: FAILED. 2586 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out; finished in 113.71s

error: test failed, to rerun pass '--test testsuite'
Build completed unsuccessfully in 0:31:53
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
