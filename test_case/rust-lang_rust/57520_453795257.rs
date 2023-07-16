plain
travis_time:end:01a24622:start=1547342162023436215,finish=1547342243260738161,duration=81237301946
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:21] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/convert-quotes.sh:2: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/nostarch.sh:2: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/tools/src/bin/convert_quotes.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/tools/src/bin/remove_links.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/tools/src/bin/link2print.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/tools/src/bin/concat_chapters.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/tools/src/bin/remove_markup.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/tools/src/bin/lfp.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/second-edition/doc-to-md.sh:2: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/convert-quotes.sh:2: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/nostarch.sh:2: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/ci/build.sh:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/ci/stable-check/src/main.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/ci/spellcheck.sh:2: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/2018-edition/convert-quotes.sh:2: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/2018-edition/nostarch.sh:2: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/2018-edition/tools/src/bin/convert_quotes.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/2018-edition/tools/src/bin/remove_links.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/2018-edition/tools/src/bin/link2print.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/2018-edition/tools/src/bin/concat_chapters.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/2018-edition/tools/src/bin/remove_markup.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/2018-edition/tools/src/bin/lfp.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/tools/src/bin/convert_quotes.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/tools/src/bin/remove_links.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/tools/src/bin/link2print.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/tools/src/bin/concat_chapters.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/tools/src/bin/remove_markup.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/book/tools/src/bin/lfp.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:21] tidy error: /checkout/src/doc/reference/stable-check/src/main.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:22] tidy error: /checkout/src/test/ui/feature-gates/feature-gate-type_alias_enum_variants.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:22] tidy error: /checkout/src/test/rustdoc/no-crate-filter.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:22] tidy error: /checkout/src/test/compile-fail-fulldeps/auxiliary/lint_group_plugin_test.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:22] tidy error: /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:22] tidy error: /checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:23] 
[00:03:23] 
[00:03:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:23] 
[00:03:23] 
[00:03:23] some tidy checks failed
[00:03:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:23] Build completed unsuccessfully in 0:00:48
[00:03:23] Makefile:69: recipe for target 'tidy' failed
[00:03:23] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f142463
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 13 01:20:55 UTC 2019
---
travis_time:end:091bf60a:start=1547342456021201712,finish=1547342456026292739,duration=5091027
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01a689b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2ea5549c
travis_time:start:2ea5549c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e0f4ecc
$ dmesg | grep -i kill
