plain
[00:57:26] .................................................................................................... 3300/4932
[00:57:30] .................................................................................................... 3400/4932
[00:57:32] ....i............................................................................................... 3500/4932
[00:57:34] .......................i............................................................................ 3600/4932
[00:57:35] ........F...................................................................i..F.................... 3700/4932
[00:57:39] .................................................................................................... 3900/4932
[00:57:42] .................................................................................................... 4000/4932
[00:57:45] .................................................................................................... 4100/4932
[00:57:49] ...................................i................................................................ 4200/4932
---
[00:58:12] failures:
[00:58:12] 
[00:58:12] ---- [ui] ui/parser/issue-5806.rs stdout ----
[00:58:12] normalized stderr:
[00:58:12] error: couldn't read $DIR/../compile-fail: No such file or directory (os error 2)
[00:58:12]    |
[00:58:12]    |
[00:58:12] LL | mod foo; //~ ERROR: a directory
[00:58:12] 
[00:58:12] error: aborting due to previous error
[00:58:12] 
[00:58:12] 
[00:58:12] 
[00:58:12] 
[00:58:12] 
[00:58:12] The actual stderr differed from the expected stderr.
[00:58:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-5806/issue-5806.stderr
[00:58:12] To update references, rerun the tests and pass the `--bless` flag
[00:58:12] To only update this specific test, also pass `--test-args parser/issue-5806.rs`
[00:58:12] error: 1 errors occurred comparing output.
[00:58:12] status: exit code: 1
[00:58:12] status: exit code: 1
[00:58:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-5806.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-5806/a" "-Crpath" "-O" "-Zunstab
