
[01:55:30] ---- doc::output_not_captured stdout ----
[01:55:30] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
[01:55:30] thread 'doc::output_not_captured' panicked at '
[01:55:30] Expected: execs
[01:55:30]     but: expected to find:
[01:55:30] 1 | â˜ƒ
[01:55:30] 
[01:55:30] did not find in output:
[01:55:30]     Checking a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t534/foo/a)
[01:55:30]  Documenting a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t534/foo/a)
[01:55:30] warning: could not parse code block as Rust code
[01:55:30]  --> a/src/lib.rs:2:17
[01:55:30]   |
[01:55:30] 2 |               /// 