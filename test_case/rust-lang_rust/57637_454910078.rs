plain
[01:47:28] test workspaces::ws_warn_unused ... ok
[01:47:28] 
[01:47:28] failures:
[01:47:28] 
[01:47:28] ---- doc::output_not_captured stdout ----
[01:47:28] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
[01:47:28] thread 'doc::output_not_captured' panicked at '
[01:47:28]     but: expected to find:
[01:47:28] 1 | ☃
[01:47:28] 
[01:47:28] did not find in output:
[01:47:28] did not find in output:
[01:47:28]  Documenting a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t534/foo/a)
[01:47:28]     Checking a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t534/foo/a)
[01:47:28] warning: could not parse code block as Rust code
[01:47:28]  --> a/src/lib.rs:2:17
[01:47:28] 2 |               /// 