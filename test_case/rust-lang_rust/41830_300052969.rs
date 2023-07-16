
[01:11:08] failures:
[01:11:08] 
[01:11:08] ---- cargo_compile_incremental stdout ----
[01:11:08] 	libdir: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib"
[01:11:08] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -v`
[01:11:08] thread 'cargo_compile_incremental' panicked at '
[01:11:08] Expected: execs
[01:11:08]     but: expected to find:
[01:11:08] [RUNNING] `rustc [..] -Zincremental=[..][/]target[/]debug[/]incremental`
[01:11:08] 
[01:11:08] 
[01:11:08] did not find in output:
[01:11:08]    Compiling foo v0.5.0 (file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t11/foo)
[01:11:08]      Running `rustc --crate-name foo src/foo.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=e1a5230cf4545c49 -C extra-filename=-e1a5230cf4545c49 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t11/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t11/foo/target/debug/deps`
[01:11:08]     Finished dev [unoptimized + debuginfo] target(s) in 0.57 secs
[01:11:08] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31
[01:11:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:08] 
[01:11:08] 
[01:11:08] failures:
[01:11:08]     cargo_compile_incremental
[01:11:08] 
[01:11:08] test result: FAILED. 98 passed; 1 failed; 0 ignored; 0 measured
[01:11:08] 
[01:11:08] error: test failed, to rerun pass '--test build'
