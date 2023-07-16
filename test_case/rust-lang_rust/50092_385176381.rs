plain
[00:39:59] ....................................................................................................
[00:40:03] ....................................................................................................
[00:40:08] ....................................................................................................
[00:40:14] ....................................................................................................
[00:40:19] ........F...........................................................................................
[00:40:31] ............i.......................................................................................
[00:40:37] ............................ii......................................................................
[00:40:43] ....................................................................................................
[00:40:48] .........i....................................................................
[00:40:48] .........i....................................................................
[00:40:48] failures:
[00:40:48] 
[00:40:48] ---- [ui] ui/issue-49934.rs stdout ----
[00:40:48]  diff of stderr:
[00:40:48] 
[00:40:48] - warning: `#[derive]` on non-item statements is ignored
[00:40:48] -   --> $DIR/issue-49934.rs:16:5
[00:40:48] -    |
[00:40:48] - LL |     #[derive(Debug)] //~ WARN `#[derive]` on non-item statements is ignored
[00:40:48] -    |
[00:40:48] -    = note: this may become a hard error in a future release
[00:40:48] - 
[00:40:48] - 
[00:40:48] - warning: `#[derive]` on expressions is ignored
[00:40:48] -   --> $DIR/issue-49934.rs:19:13
[00:40:48] -    |
[00:40:48] - LL |     let _ = #[derive(Debug)] "Hello, world!";
[00:40:48] -    |
[00:40:48] -    = note: this may become a hard error in a future release
[00:40:48] - 
[00:40:48] - 
[00:40:48] - warning: `#[derive]` on expressions is ignored
[00:40:48] -   --> $DIR/issue-49934.rs:23:9
[00:40:48] -    |
[00:40:48] - LL |         #[derive(Debug)] //~ WARN `#[derive]` on expressions is ignored
[00:40:48] -    |
[00:40:48] -    = note: this may become a hard error in a future release
[00:40:48] - 
[00:40:48] - 
---
[00:40:48] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-49934.rs'
[00:40:48] 
[00:40:48] error: 1 errors occurred comparing output.
[00:40:48] status: exit code: 0
[00:40:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-49934.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-49934.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-49934.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
