plain
travis_time:end:0a89da50:start=1556031816357528291,finish=1556032985477281785,duration=1169119753494
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:43] 
[01:18:43] running 9 tests
[01:18:43] iiiiiiiii
[01:18:43] 
[01:18:43]  finished in 0.158
[01:18:43] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:00] 
[01:19:00] running 121 tests
[01:19:26] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:19:31] i.i......iii.i.....ii
[01:19:31] 
[01:19:31]  finished in 30.805
[01:19:31] travis_fold:end:test_debuginfo

---
[01:26:58] 
[01:26:58] running 53 tests
[01:27:00] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:27:00] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:27:00] .........................................F...........
[01:27:00] 
[01:27:00] ---- [pretty] pretty/stmt_expr_attributes.rs stdout ----
[01:27:00] 
[01:27:00] error: pretty-printing failed in round 0 revision None
[01:27:00] error: pretty-printing failed in round 0 revision None
[01:27:00] status: exit code: 1
[01:27:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/pretty/stmt_expr_attributes.rs" "-Z" "unpretty=normal" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/stmt_expr_attributes/auxiliary.pretty" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:27:00] ------------------------------------------
[01:27:00] ------------------------------------------
[01:27:00] // pp-exact
[01:27:00] #![feature(custom_attribute)]
[01:27:00] #![feature(custom_attribute)]
[01:27:00] #![feature(box_syntax)]
[01:27:00] #![feature(stmt_expr_attributes)]
[01:27:00] fn main() { }
[01:27:00] 
[01:27:00] 
[01:27:00] fn _0() {
[01:27:00]     #[attr]
[01:27:00]     foo();
[01:27:00] }
[01:27:00] 
[01:27:00] 
[01:27:00] fn _1() {
[01:27:00]     #[attr]
[01:27:00]     unsafe {
[01:27:00]         // code
[01:27:00]     }
[01:27:00]     }
[01:27:00] }
[01:27:00] 
[01:27:00] fn _2() {
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     { foo(); }
[01:27:00]     {
[01:27:00]     {
[01:27:00]         #![attr]
[01:27:00]         foo()
[01:27:00]     }
[01:27:00] }
[01:27:00] 
[01:27:00] 
[01:27:00] fn _3() {
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     match () { _ => { } }
[01:27:00] }
[01:27:00] 
[01:27:00] fn _4() {
[01:27:00]     #[attr]
[01:27:00]     match () {
[01:27:00]     match () {
[01:27:00]         #![attr]
[01:27:00]     }
[01:27:00] 
[01:27:00]     let _ =
[01:27:00]     let _ =
[01:27:00]         #[attr] match () {
[01:27:00]                     #![attr]
[01:27:00]                     () => (),
[01:27:00] }
[01:27:00] 
[01:27:00] 
[01:27:00] fn _5() {
[01:27:00]     #[attr]
[01:27:00]     let x = 1;
[01:27:00] 
[01:27:00] 
[01:27:00]     let x = #[attr] 1;
[01:27:00]     let y = ();
[01:27:00]     let z = ();
[01:27:00] 
[01:27:00] 
[01:27:00]     foo3(x, #[attr] y, z);
[01:27:00] 
[01:27:00]     qux(3 + #[attr] 2);
[01:27:00] }
[01:27:00] 
[01:27:00] fn _6() {
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     [#![attr] 1, 2, 3];
[01:27:00] 
[01:27:00]     let _ = #[attr] [#![attr] 1, 2, 3];
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     [#![attr] 1; 4];
[01:27:00] 
[01:27:00]     let _ = #[attr] [#![attr] 1; 4];
[01:27:00] }
[01:27:00] struct Foo {
[01:27:00]     data: (),
[01:27:00] }
[01:27:00] 
[01:27:00] 
[01:27:00] struct Bar(());
[01:27:00] 
[01:27:00] fn _7() {
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     Foo{#![attr] data: (),};
[01:27:00] 
[01:27:00]     let _ = #[attr] Foo{#![attr] data: (),};
[01:27:00] }
[01:27:00] 
[01:27:00] fn _8() {
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     (#![attr] );
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     (#![attr] 0);
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     (#![attr] 0,);
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     (#![attr] 0, 1);
[01:27:00] }
[01:27:00] 
[01:27:00] fn _9() {
[01:27:00]     macro_rules! stmt_mac((  ) => { let _ = (  ) ; });
[01:27:00]     #[attr]
[01:27:00]     stmt_mac!();
[01:27:00] 
[01:27:00]     /*
[01:27:00]     /*
[01:27:00]     // pre existing pp bug: delimiter styles gets lost:
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     stmt_mac!{ };
[01:27:00]     #[attr]
[01:27:00]     stmt_mac![];
[01:27:00] 
[01:27:00]     #[attr]
[01:27:00]     #[attr]
[01:27:00]     stmt_mac!{ } // pre-existing pp bug: compiler ICEs with a None unwrap
[01:27:00]     */
[01:27:00]     let _ = ();
[01:27:00] }
[01:27:00] 
[01:27:00] 
[01:27:00] macro_rules! expr_mac((  ) => { (  ) });
[01:27:00] 
[01:27:00] fn _10() {
[01:27:00] 
[01:27:00]     let _ = #[attr] expr_mac!();
[01:27:00]     /*
[01:27:00]     /*
[01:27:00]     // pre existing pp bug: delimiter styles gets lost:
[01:27:00]     let _ = #[attr] expr_mac![];
[01:27:00]     let _ = #[attr] expr_mac!{};
[01:27:00]     */
[01:27:00] }
[01:27:00] 
[01:27:00] fn _11() {
[01:27:00]     let _ = #[attr] box 0;
[01:27:00]     let _: [(); 0] = #[attr] [#![attr] ];
[01:27:00]     let _ = #[attr] [#![attr] 0, 0];
[01:27:00]     let _ = #[attr] [#![attr] 0; 0];
[01:27:00]     let _ = #[attr] foo();
[01:27:00]     let _ = #[attr] 1i32.clone();
[01:27:00]     let _ = #[attr] (#![attr] );
[01:27:00]     let _ = #[attr] (#![attr] 0);
[01:27:00]     let _ = #[attr] (#![attr] 0,);
[01:27:00]     let _ = #[attr] (#![attr] 0, 0);
[01:27:00]     let _ = #[attr] 0 + #[attr] 0;
[01:27:00]     let _ = #[attr] !0;
[01:27:00]     let _ = #[attr] -0i32;
[01:27:00]     let _ = #[attr] false;
[01:27:00]     let _ = #[attr] 'c';
[01:27:00]     let _ = #[attr] 0;
[01:27:00]     let _ = #[attr] 0 as usize;
[01:27:00]     let _ =
[01:27:00]         #[attr] while false {
[01:27:00]                     #![attr]
[01:27:00]     let _ =
[01:27:00]     let _ =
[01:27:00]         #[attr] while let None = Some(()) {
[01:27:00]                     #![attr]
[01:27:00]     let _ =
[01:27:00]     let _ =
[01:27:00]         #[attr] for _ in 0..0 {
[01:27:00]                     #![attr]
[01:27:00]                 };
[01:27:00]     // FIXME: pp bug, two spaces after the loop
[01:27:00]     let _ =
[01:27:00]         #[attr] loop  {
[01:27:00]                     #![attr]
[01:27:00]     let _ =
[01:27:00]     let _ =
[01:27:00]         #[attr] match false {
[01:27:00]                     #![attr]
[01:27:00]                 };
[01:27:00]                 };
[01:27:00]     let _ = #[attr] || #[attr] ();
[01:27:00]     let _ = #[attr] move || #[attr] ();
[01:27:00]     let _ =
[01:27:00]         #[attr] ||
[01:27:00]                     {
[01:27:00]                         #![attr]
[01:27:00]                         #[attr]
[01:27:00]                         ()
[01:27:00]     let _ =
[01:27:00]     let _ =
[01:27:00]         #[attr] move ||
[01:27:00]                     {
[01:27:00]                         #![attr]
[01:27:00]                         #[attr]
[01:27:00]                         ()
[01:27:00]     let _ =
[01:27:00]         #[attr] {
[01:27:00]         #[attr] {
[01:27:00]                     #![attr]
[01:27:00]     let _ =
[01:27:00]         #[attr] {
[01:27:00]         #[attr] {
[01:27:00]                     #![attr]
[01:27:00]                     let _ = ();
[01:27:00]     let _ =
[01:27:00]         #[attr] {
[01:27:00]         #[attr] {
[01:27:00]                     #![attr]
[01:27:00]                     let _ = ();
[01:27:00]                     ()
[01:27:00]     let mut x = 0;
[01:27:00]     let mut x = 0;
[01:27:00]     let _ = #[attr] x = 15;
[01:27:00]     let _ = #[attr] x += 15;
[01:27:00]     let s = Foo{data: (),};
[01:27:00]     let _ = #[attr] s.data;
[01:27:00]     let _ = (#[attr] s).data;
[01:27:00]     let t = Bar(());
[01:27:00]     let _ = #[attr] t.0;
[01:27:00]     let _ = (#[attr] t).0;
[01:27:00]     let v = vec!(0);
[01:27:00]     let _ = #[attr] v[0];
[01:27:00]     let _ = (#[attr] v)[0];
[01:27:00]     let _ = #[attr] 0..#[attr] 0;
[01:27:00]     let _ = #[attr] 0..;
[01:27:00]     let _ = #[attr] (0..0);
[01:27:00]     let _ = #[attr] (0..);
[01:27:00]     let _ = #[attr] (..0);
[01:27:00]     let _ = #[attr] (..);
[01:27:00]     let _: fn(&u32) -> u32 = #[attr] std::clone::Clone::clone;
[01:27:00]     let _ = #[attr] &0;
[01:27:00]     let _ = #[attr] &mut 0;
[01:27:00]     let _ = #[attr] &#[attr] 0;
[01:27:00]     let _ = #[attr] &mut #[attr] 0;
[01:27:00]     // FIXME: pp bug, extra space after keyword?
[01:27:00]     while false { let _ = #[attr] continue ; }
[01:27:00]     while true { let _ = #[attr] break ; } || (#[attr] return);
[01:27:00]     let _ = #[attr] expr_mac!();
[01:27:00]     /* FIXME: pp bug, losing delimiter styles
[01:27:00]     let _ = #[attr] expr_mac![];
[01:27:00]     let _ = #[attr] expr_mac!{};
[01:27:00]     */
[01:27:00]     let _ = #[attr] Foo{#![attr] data: (),};
[01:27:00]     let _ = #[attr] Foo{#![attr] ..s};
[01:27:00]     let _ = #[attr] Foo{#![attr] data: (), ..s};
[01:27:00]     let _ = #[attr] (#![attr] 0);
[01:27:00] }
[01:27:00] 
[01:27:00] fn _12() {
[01:27:00]     #[attr]
[01:27:00] 
[01:27:00]     #[attr]
[01:27:00]     0;
[01:27:00] 
[01:27:00] 
[01:27:00]     #[attr]
[01:27:00]     expr_mac!();
[01:27:00] 
[01:27:00]     #[attr]
[01:27:00]     {
[01:27:00]         #![attr]
[01:27:00] }
[01:27:00] 
[01:27:00] 
[01:27:00] /////////////////
[01:27:00] fn foo() { }
[01:27:00] fn foo() { }
[01:27:00] fn foo3(_: i32, _: (), _: ()) { }
[01:27:00] fn qux(_: i32) { }
[01:27:00] ------------------------------------------
[01:27:00] stderr:
[01:27:00] ------------------------------------------
[01:27:00] error: ambiguous parse
[01:27:00] error: ambiguous parse
[01:27:00]    --> /checkout/src/test/pretty/stmt_expr_attributes.rs:245:5
[01:27:00]     |
[01:27:00] 244 |     while true { let _ = #[attr] break ; }
[01:27:00]     |     -------------------------------------- help: parenthesis are required to parse this as an expression: `(while true { let _ = #[attr] break ; })`
[01:27:00] 245 |     || #[attr] return;
[01:27:00] 
[01:27:00] error: aborting due to previous error
[01:27:00] 
[01:27:00] 
---
[01:27:00] test result: FAILED. 52 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:27:00] 
[01:27:00] 
[01:27:00] 
[01:27:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:27:00] 
[01:27:00] 
[01:27:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:00] Build completed unsuccessfully in 0:20:13
[01:27:00] Build completed unsuccessfully in 0:20:13
[01:27:00] make: *** [check] Error 1
[01:27:00] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:306169a7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 16:50:15 UTC 2019
---
travis_time:end:027d71c0:start=1556038217385687382,finish=1556038217390664582,duration=4977200
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01e18760
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:335bc6dd
travis_time:start:335bc6dd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19800ad2
$ dmesg | grep -i kill
