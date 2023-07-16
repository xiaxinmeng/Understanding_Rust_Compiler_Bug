plain
[00:44:06] .................................................................................................... 1400/4623
[00:44:09] .................................................................i.................................. 1500/4623
[00:44:12] ................................i................................................................... 1600/4623
[00:44:15] .................................................................................................... 1700/4623
[00:44:18] ...............F.................................................................................... 1800/4623
[00:44:21] ...................................................................i................................ 1900/4623
[00:44:25] .........F.......................................................................................... 2000/4623
[00:44:32] .................................................................................................... 2200/4623
[00:44:36] .................................................................................................... 2300/4623
[00:44:39] .................................................................................................... 2400/4623
[00:44:43] .................................................................................................... 2500/4623
---
[00:45:50] 
[00:45:50] ---- [ui] ui/issue-52717.rs stdout ----
[00:45:50] diff of stderr:
[00:45:50] 
[00:45:50] 1 error[E0026]: variant `A::A` does not have a field named `fob`
[00:45:50] +   --> $DIR/issue-52717.rs:19:12
[00:45:50] 3    |
[00:45:50] 3    |
[00:45:50] 4 LL |     A::A { fob } => { println!("{}", fob); }
[00:45:50] 
[00:45:50] 
[00:45:50] The actual stderr differed from the expected stderr.
[00:45:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-52717/issue-52717.stderr
[00:45:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-52717/issue-52717.stderr
[00:45:50] To update references, rerun the tests and pass the `--bless` flag
[00:45:50] To only update this specific test, also pass `--test-args issue-52717.rs`
[00:45:50] error: 1 errors occurred comparing output.
[00:45:50] status: exit code: 1
[00:45:50] status: exit code: 1
[00:45:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-52717.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-52717/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-52717/auxiliary" "-A" "unused"
[00:45:50] ------------------------------------------
[00:45:50] 
[00:45:50] ------------------------------------------
[00:45:50] stderr:
[00:45:50] stderr:
[00:45:50] ------------------------------------------
[00:45:50] {"message":"variant `A::A` does not have a field named `fob`","code":{"code":"E0026","explanation":"\nThis error indicates that a struct pattern attempted to extract a non-existent\nfield from a struct. Struct fields are identified by the name used before the\ncolon `:` so struct patterns should resemble the declaration of the struct type\nbeing matched.\n\n