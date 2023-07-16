plain
[00:46:37] ....................................................................................................
[00:46:40] ....................................................................................................
[00:46:42] ....................................................................................................
[00:46:46] ....................................................................................................
[00:46:49] .........F..........................................................................................
[00:46:55] ..........................................................................................i.........
[00:46:58] ................................................................................i...................
[00:47:02] ....................................................................................................
[00:47:05] ....................................................................................................
---
[00:47:14] 
[00:47:14] ---- [ui] ui/issue-43926.rs stdout ----
[00:47:14] diff of stderr:
[00:47:14] 
[00:47:14] 1 error: `cfg()` must have an argument
[00:47:14] +   --> $DIR/issue-43926.rs:11:20
[00:47:14] 3    |
[00:47:14] 3    |
[00:47:14] 4 LL | #[link(name="foo", cfg())] //~ ERROR `cfg()` must have an argument
[00:47:14] +    |                    ^^^^^
[00:47:14] 6 
[00:47:14] 7 error: aborting due to previous error
[00:47:14] 8 
[00:47:14] 8 
[00:47:14] 
[00:47:14] 
[00:47:14] The actual stderr differed from the expected stderr.
[00:47:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-43926/issue-43926.stderr
[00:47:14] To update references, rerun the tests and pass the `--bless` flag
[00:47:14] To only update this specific test, also pass `--test-args issue-43926.rs`
[00:47:14] error: 1 errors occurred comparing output.
[00:47:14] status: exit code: 101
[00:47:14] status: exit code: 101
[00:47:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-43926.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-43926/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build
