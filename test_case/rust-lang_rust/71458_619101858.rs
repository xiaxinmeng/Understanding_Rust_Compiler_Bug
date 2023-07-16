
failures:

---- rustdocflags::parses_config stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -v`
thread 'rustdocflags::parses_config' panicked at '
Expected: execs
    but: expected to find:
[RUNNING] `rustdoc [..] --cfg foo[..]`

did not find in output:
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1536/foo)
     Running `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1536/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1536/foo/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 1.00s
', src/tools/cargo/crates/cargo-test-support/src/lib.rs:833:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    rustdocflags::parses_config

test result: FAILED. 1841 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--test testsuite'
