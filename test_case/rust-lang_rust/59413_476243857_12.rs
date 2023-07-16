\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/bindings.rs","byte_start":551,"byte_end":552,"line_start":24,"line_end":24,"column_start":33,"column_end":34,"is_primary":true,"text":[{"text":"        const foo: impl Clone = x;","highlight_start":33,"highlight_end":34}],"label":"non-constant value","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0435]: attempt to use a non-constant value in a constant\n  --> /checkout/src/test/ui/impl-trait/bindings.rs:24:33\n   |\nLL |         const foo: impl Clone = x;\n   |                                 ^ non-constant value\n\n"}
[01:10:51] thread 'rustc' panicked at 'expected node_id to be lowered already Item {
[01:10:51]     ident: foo#0,
[01:10:51]     attrs: [],
[01:10:51]     id: NodeId(21),
[01:10:51]     node: Const(
[01:10:51]         type(impl Clone),
[01:10:51]         expr(27: x)
[01:10:51]     vis: Spanned {
[01:10:51]         node: Inherited,
[01:10:51]         span: Span {
[01:10:51]         span: Span {
[01:10:51]             lo: BytePos(
[01:10:51]             ),
[01:10:51]             ),
[01:10:51]             hi: BytePos(
[01:10:51]             ),
[01:10:51]             ),
[01:10:51]             ctxt: #0
[01:10:51]     },
[01:10:51]     span: Span {
[01:10:51]     span: Span {
[01:10:51]         lo: BytePos(
[01:10:51]         ),
[01:10:51]         ),
[01:10:51]         hi: BytePos(
[01:10:51]         ),
[01:10:51]         ),
[01:10:51]         ctxt: #0
[01:10:51]     tokens: Some(
[01:10:51]         TokenStream(
[01:10:51]             Some(
[01:10:51]                 [
[01:10:51]                 [
[01:10:51]                     (
[01:10:51]                         Token(
[01:10:51]                             Span {
[01:10:51]                                 lo: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 hi: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 ctxt: #0
[01:10:51]                             Ident(
[01:10:51]                                 const#0,
[01:10:51]                                 false
[01:10:51]                             )
[01:10:51]                             )
[01:10:51]                         ),
[01:10:51]                         NonJoint
[01:10:51]                     ),
[01:10:51]                     (
[01:10:51]                         Token(
[01:10:51]                             Span {
[01:10:51]                                 lo: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 hi: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 ctxt: #0
[01:10:51]                             Ident(
[01:10:51]                                 foo#0,
[01:10:51]                                 false
[01:10:51]                             )
[01:10:51]                             )
[01:10:51]                         ),
[01:10:51]                         NonJoint
[01:10:51]                     ),
[01:10:51]                     (
[01:10:51]                         Token(
[01:10:51]                             Span {
[01:10:51]                                 lo: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 hi: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 ctxt: #0
[01:10:51]                             Colon
[01:10:51]                         ),
[01:10:51]                         NonJoint
[01:10:51]                     ),
[01:10:51]                     ),
[01:10:51]                     (
[01:10:51]                         Token(
[01:10:51]                             Span {
[01:10:51]                                 lo: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 hi: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 ctxt: #0
[01:10:51]                             Ident(
[01:10:51]                                 impl#0,
[01:10:51]                                 false
[01:10:51]                             )
[01:10:51]                             )
[01:10:51]                         ),
[01:10:51]                         NonJoint
[01:10:51]                     ),
[01:10:51]                     (
[01:10:51]                         Token(
[01:10:51]                             Span {
[01:10:51]                                 lo: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 hi: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 ctxt: #0
[01:10:51]                             Ident(
[01:10:51]                                 Clone#0,
[01:10:51]                                 false
[01:10:51]                             )
[01:10:51]                             )
[01:10:51]                         ),
[01:10:51]                         NonJoint
[01:10:51]                     ),
[01:10:51]                     (
[01:10:51]                         Token(
[01:10:51]                             Span {
[01:10:51]                                 lo: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 hi: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 ctxt: #0
[01:10:51]                             Eq
[01:10:51]                         ),
[01:10:51]                         NonJoint
[01:10:51]                     ),
[01:10:51]                     ),
[01:10:51]                     (
[01:10:51]                         Token(
[01:10:51]                             Span {
[01:10:51]                                 lo: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 hi: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 ctxt: #0
[01:10:51]                             Ident(
[01:10:51]                                 x#0,
[01:10:51]                                 false
[01:10:51]                             )
[01:10:51]                             )
[01:10:51]                         ),
[01:10:51]                         NonJoint
[01:10:51]                     ),
[01:10:51]                     (
[01:10:51]                         Token(
[01:10:51]                             Span {
[01:10:51]                                 lo: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 hi: BytePos(
[01:10:51]                                 ),
[01:10:51]                                 ),
[01:10:51]                                 ctxt: #0
[01:10:51]                             Semi
[01:10:51]                         ),
[01:10:51]                         NonJoint
[01:10:51]                     )
---
[01:10:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:51] 
[01:10:51] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:10:51] 
[01:10:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:10:51] 
[01:10:51] ------------------------------------------
[01:10:51] 
[01:10:51] thread '[ui] ui/impl-trait/bindings.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
---
[01:10:51] 
[01:10:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:10:51] 
[01:10:51] 
[01:10:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:51] 
[01:10:51] 
[01:10:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:51] Build completed unsuccessfully in 0:04:13
[01:10:51] Build completed unsuccessfully in 0:04:13
[01:10:51] make: *** [check] Error 1
[01:10:51] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e5f1986
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 15:18:26 UTC 2019
---
travis_time:end:0aa41be2:start=1553527107629037674,finish=1553527107633557330,duration=4519656
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:100cd623
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:aft
