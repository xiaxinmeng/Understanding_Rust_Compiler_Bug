plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +30f9ae62bdb7ea7c646fd304b948a3344779405f:refs/remotes/pull/80080/merge
---
.................................................................................................... 9000/11170
.................................................................................................... 9100/11170
...............................................................i......i............................. 9200/11170
.................................................................................................... 9300/11170
..iiiiii..iiiiii.i.................................................................................. 9400/11170
.................................................................................................... 9600/11170
.................................................................................................... 9700/11170
.............................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
....................................... 9800/11170
---

---- [ui] ui/parser/brace-after-qualified-path-in-match.rs stdout ----
diff of stderr:

- error: unexpected `{` after qualified path
-   --> $DIR/brace-after-qualified-path-in-match.rs:3:27
+ error[E0433]: failed to resolve: use of undeclared type `Trait`
+   --> $DIR/brace-after-qualified-path-in-match.rs:3:15
3    |
4 LL |         <T as Trait>::Type{key: value} => (),
-    |         ------------------^ unexpected `{` after qualified path
-    |         |
-    |         the qualified path
+    |               ^^^^^ use of undeclared type `Trait`
- error: aborting due to previous error
+ error[E0412]: cannot find type `T` in this scope
+   --> $DIR/brace-after-qualified-path-in-match.rs:3:10
+    |
+    |
+ LL |         <T as Trait>::Type{key: value} => (),
+    |          ^ not found in this scope
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0412, E0433.
+ For more information about an error, try `rustc --explain E0412`.
+ For more information about an error, try `rustc --explain E0412`.
11 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/brace-after-qualified-path-in-match/brace-after-qualified-path-in-match.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/brace-after-qualified-path-in-match.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/brace-after-qualified-path-in-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/brace-after-qualified-path-in-match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/brace-after-qualified-path-in-match/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type `Trait`
  --> /checkout/src/test/ui/parser/brace-after-qualified-path-in-match.rs:3:15
   |
LL |         <T as Trait>::Type{key: value} => (),
   |               ^^^^^ use of undeclared type `Trait`
error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/parser/brace-after-qualified-path-in-match.rs:3:10
   |
   |
LL |         <T as Trait>::Type{key: value} => (),
   |          ^ not found in this scope
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.

------------------------------------------


---- [ui] ui/parser/paren-after-qualified-path-in-match.rs stdout ----
diff of stderr:

- error: unexpected `(` after qualified path
-   --> $DIR/paren-after-qualified-path-in-match.rs:3:27
+ error[E0433]: failed to resolve: use of undeclared type `Trait`
+   --> $DIR/paren-after-qualified-path-in-match.rs:3:15
3    |
4 LL |         <T as Trait>::Type(2) => (),
-    |         ------------------^ unexpected `(` after qualified path
-    |         |
-    |         the qualified path
+    |               ^^^^^ use of undeclared type `Trait`
- error: aborting due to previous error
+ error[E0412]: cannot find type `T` in this scope
+   --> $DIR/paren-after-qualified-path-in-match.rs:3:10
+    |
+    |
+ LL |         <T as Trait>::Type(2) => (),
+    |          ^ not found in this scope
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0412, E0433.
+ For more information about an error, try `rustc --explain E0412`.
+ For more information about an error, try `rustc --explain E0412`.
11 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/paren-after-qualified-path-in-match/paren-after-qualified-path-in-match.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/paren-after-qualified-path-in-match.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/paren-after-qualified-path-in-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/paren-after-qualified-path-in-match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/paren-after-qualified-path-in-match/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type `Trait`
  --> /checkout/src/test/ui/parser/paren-after-qualified-path-in-match.rs:3:15
   |
LL |         <T as Trait>::Type(2) => (),
   |               ^^^^^ use of undeclared type `Trait`
error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/parser/paren-after-qualified-path-in-match.rs:3:10
   |
   |
LL |         <T as Trait>::Type(2) => (),
   |          ^ not found in this scope
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.

------------------------------------------


---- [ui] ui/suggestions/vec-macro-in-pattern.rs stdout ----
diff of stderr:

- error: unexpected `(` after qualified path
+ error: arbitrary expressions aren't allowed in patterns
+    |
+    |
+ LL |         Some(vec![_x]) => (),
+ 
+ 
+ error[E0425]: cannot find value `_x` in this scope
+    |
+    |
+ LL |         Some(vec![_x]) => (),
+    |                   ^^ not found in this scope
+ 
+ error[E0658]: box pattern syntax is experimental
3    |
3    |
4 LL |         Some(vec![_x]) => (),
5    |              ^^^^^^^^
-    |              |
-    |              |
-    |              unexpected `(` after qualified path
-    |              the qualified path
-    |              in this macro invocation
-    |              help: use a slice pattern here instead: `[_x]`
11    |
-    = help: for more information, see https://doc.rust-lang.org/edition-guide/rust-2018/slice-patterns.html
+    = help: add `#![feature(box_patterns)]` to the crate attributes to enable
13    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
14 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0164]: expected tuple struct or tuple variant, found associated function `<[_]>::into_vec`
+    |
+    |
+ LL |         Some(vec![_x]) => (),
+    |              ^^^^^^^^ `fn` calls are not allowed in patterns
+    |
+    = help: for more information, visit https://doc.rust-lang.org/book/ch18-00-patterns.html
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to 4 previous errors
+ 
+ Some errors have detailed explanations: E0164, E0425, E0658.
+ For more information about an error, try `rustc --explain E0164`.
+ For more information about an error, try `rustc --explain E0164`.
17 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/vec-macro-in-pattern/vec-macro-in-pattern.stderr
diff of fixed:
2 fn main() {
2 fn main() {
3     // everything after `.as_ref` should be suggested
4     match Some(vec![3]).as_ref().map(|v| v.as_slice()) {
-         Some([_x]) => (), //~ ERROR unexpected `(` after qualified path
+         Some(vec![_x]) => (), //~ ERROR unexpected `(` after qualified path
6         _ => (),
8 }


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/vec-macro-in-pattern/vec-macro-in-pattern.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/vec-macro-in-pattern.rs`
error: 2 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/vec-macro-in-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/vec-macro-in-pattern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/vec-macro-in-pattern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: arbitrary expressions aren't allowed in patterns
   |
   |
LL |         Some(vec![_x]) => (), //~ ERROR unexpected `(` after qualified path


error[E0425]: cannot find value `_x` in this scope
   |
   |
LL |         Some(vec![_x]) => (), //~ ERROR unexpected `(` after qualified path
   |                   ^^ not found in this scope

error[E0658]: box pattern syntax is experimental
   |
   |
LL |         Some(vec![_x]) => (), //~ ERROR unexpected `(` after qualified path
   |
   = note: see issue #29641 <https://github.com/rust-lang/rust/issues/29641> for more information
   = help: add `#![feature(box_patterns)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0164]: expected tuple struct or tuple variant, found associated function `<[_]>::into_vec`
   |
   |
LL |         Some(vec![_x]) => (), //~ ERROR unexpected `(` after qualified path
   |              ^^^^^^^^ `fn` calls are not allowed in patterns
   |
   = help: for more information, visit https://doc.rust-lang.org/book/ch18-00-patterns.html
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0164, E0425, E0658.
For more information about an error, try `rustc --explain E0164`.
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:11
