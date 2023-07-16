plain
.................................................................................................... 9000/11196
.................................................................................................... 9100/11196
......................................................................................i......i...... 9200/11196
.................................................................................................... 9300/11196
.........................iiiiii..iiiiii.i........................................................... 9400/11196
.................................................................................................... 9600/11196
.................................................................................................... 9700/11196
................................................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
.................... 9800/11196
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.081 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 12.673 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i....ii.i.i....ii..........iiii..........i....i....i......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.42s

 finished in 2.499 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

127 warning: could not parse code block as Rust code
128   --> $DIR/invalid-syntax.rs:92:9
129    |
- LL | ///     \____/
-    |         ^^^^^^
+ LL |   ///     \____/
+ LL | | ///
+    | |_
132    |
133    = note: error from rustc: unknown start of token: \
133    = note: error from rustc: unknown start of token: \
134 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/invalid-syntax.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args invalid-syntax.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: could not parse code block as Rust code
  --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:3:5
   |
LL |   /// 