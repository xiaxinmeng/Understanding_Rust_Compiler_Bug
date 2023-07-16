plain
travis_time:end:029d1bcc:start=1547695902697366800,finish=1547695973775796919,duration=71078430119
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:00] ..................................................................................i................. 3200/5303
[01:03:03] .................................................................................................... 3300/5303
[01:03:07] ...............................................ii...i..ii........................................... 3400/5303
[01:03:11] .................................................................................................... 3500/5303
[01:03:14] ...F................................................................................................ 3600/5303
[01:03:19] .........................................................i.......................................... 3800/5303
[01:03:22] .................................................................................................... 3900/5303
[01:03:24] .............i...................................................................................... 4000/5303
[01:03:27] .................................................................................................... 4100/5303
---
[01:04:16] 
[01:04:16] ---- [ui] ui/nll/issue-55401.rs stdout ----
[01:04:16] diff of stderr:
[01:04:16] 
[01:04:16] - error: unsatisfied lifetime constraints
[01:04:16] + error: lifetime may not live long enough
[01:04:16] 3    |
[01:04:16] 3    |
[01:04:16] 4 LL | fn static_to_a_to_static_through_ref_in_tuple<'a>(x: &'a u32) -> &'static u32 {
[01:04:16] 
[01:04:16] The actual stderr differed from the expected stderr.
[01:04:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55401/issue-55401.stderr
[01:04:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55401/issue-55401.stderr
[01:04:16] To update references, rerun the tests and pass the `--bless` flag
[01:04:16] To only update this specific test, also pass `--test-args nll/issue-55401.rs`
[01:04:16] error: 1 errors occurred comparing output.
[01:04:16] status: exit code: 1
[01:04:16] status: exit code: 1
[01:04:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55401.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55401/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55401/auxiliary" "-A" "unused"
[01:04:16] ------------------------------------------
[01:04:16] 
[01:04:16] ------------------------------------------
[01:04:16] stderr:
[01:04:16] stderr:
[01:04:16] ------------------------------------------
[01:04:16] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-55401.rs","byte_start":64,"byte_end":66,"line_start":3,"line_end":3,"column_start":47,"column_end":49,"is_primary":false,"text":[{"text":"fn static_to_a_to_static_through_ref_in_tuple<'a>(x: &'a u32) -> &'static u32 {","highlight_start":47,"highlight_end":49}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-55401.rs","byte_start":151,"byte_end":153,"line_start":5,"line_end":5,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    *y //~ ERROR","highlight_start":5,"highlight_end":7}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/issue-55401.rs:5:5\n   |\nLL | fn static_to_a_to_static_through_ref_in_tuple<'a>(x: &'a u32) -> &'static u32 {\n   |                                               -- lifetime `'a` defined here\nLL |     let (ref y, _z): (&'a u32, u32) = (&22, 44);\nLL |     *y //~ ERROR\n   |     ^^ returning this value requires that `'a` must outlive `'static`\n\n"}
[01:04:16] 
[01:04:16] ------------------------------------------
[01:04:16] 
[01:04:16] thread '[ui] ui/nll/issue-55401.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
travis_time:end:04227d80:start=1547699842392598279,finish=1547699842396961988,duration=4363709
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:069b4e88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d83d7c0
travis_time:start:0d83d7c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:003e8d00
$ dmesg | grep -i kill
