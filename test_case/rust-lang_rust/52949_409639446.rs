plain
[00:42:32] ....................................................................................................
[00:42:35] ....................................................................................................
[00:42:38] ....................................................................................................
[00:42:41] ....................................................................................................
[00:42:44] ................................................F...................................i...............
[00:42:51] ....................................................................................................
[00:42:54] ....................................................................................................
[00:42:57] ....................................................................................................
[00:43:01] .
[00:43:01] .
[00:43:01] failures:
[00:43:01] 
[00:43:01] ---- [ui] ui/macros/trace-macro.rs stdout ----
[00:43:01] diff of stderr:
[00:43:01] 
[00:43:01] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:01] 6    |
[00:43:01] 7    = note: expanding `println! { "Hello, World!" }`
[00:43:01] -    = note: to `{
[00:43:01] -            # [ cfg ( not ( stage0 ) ) ] {
[00:43:01] -            ( $crate :: io :: _print ( format_args_nl ! ( "Hello, World!" ) ) ) ; } # [
[00:43:01] -            cfg ( stage0 ) ] { print ! ( "{}/n" , format_args ! ( "Hello, World!" ) ) } }`
[00:43:01] +    = note: to `{ $crate :: io :: _print ( format_args_nl ! ( "Hello, World!" ) ) ; }`
[00:43:01] 13 
[00:43:01] 
[00:43:01] 
[00:43:01] The actual stderr differed from the expected stderr.
[00:43:01] The actual stderr differed from the expected stderr.
[00:43:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace-macro/trace-macro.stderr
[00:43:01] To update references, rerun the tests and pass the `--bless` flag
[00:43:01] To only update this specific test, also pass `--test-args macros/trace-macro.rs`
[00:43:01] error: 1 errors occurred comparing output.
[00:43:01] status: exit code: 0
[00:43:01] status: exit code: 0
[00:43:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace-macro.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace-macro/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "trace-macros" 3227688 .
2269684 ./obj/build
1666164 ./obj/build/x86_64-unknown-linux-gnu
786452 ./src
466256 ./obj/build/x86_64-unknown-linux-gnu/stage0
---
149132 ./src/llvm-emscripten/test
147656 ./obj/build/bootstrap/debug/incremental
133568 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133564 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
133232 ./obj/build/bootstrap/debug/incremental/bootstrap-jdsuci5s9dha
133228 ./obj/build/bootstrap/debug/incremental/bootstrap-jdsuci5s9dha/s-f3gae0dp99-17brhro-30ju1irgdh905
128592 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122492 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu8690943357,finish=1533141558699204611,duration=8261254
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_fold:start:after_failure.4
travis_time:start:0095c6f1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c7d4117
travis_time:start:0c7d4117
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1256c09a
$ dmesg | grep -i kill
