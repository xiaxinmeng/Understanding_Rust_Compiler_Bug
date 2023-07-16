plain

---- [ui] ui/pattern/usefulness/deny-irrefutable-let-patterns.rs stdout ----
diff of stderr:

1 error: irrefutable `if let` pattern
-   --> $DIR/deny-irrefutable-let-patterns.rs:7:5
+   --> $DIR/deny-irrefutable-let-patterns.rs:6:5
4 LL |     if let _ = 5 {}
5    |     ^^^^^^^^^^^^^^^

6    |
6    |
7 note: the lint level is defined here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/deny-irrefutable-let-patterns.rs:4:9
+   --> $DIR/deny-irrefutable-let-patterns.rs:3:9
9    |
10 LL | #![deny(irrefutable_let_patterns)]


13    = help: consider replacing the `if let` with a `let`
14 
15 error: irrefutable `while let` pattern
-   --> $DIR/deny-irrefutable-let-patterns.rs:9:5
+   --> $DIR/deny-irrefutable-let-patterns.rs:8:5
17    |
18 LL | /     while let _ = 5 {
19 LL | |         break;

24    = help: consider instead using a `loop { ... }` with a `let` inside it
25 
26 error: irrefutable `if let` guard pattern
-   --> $DIR/deny-irrefutable-let-patterns.rs:14:18
+   --> $DIR/deny-irrefutable-let-patterns.rs:13:18
28    |
29 LL |         _ if let _ = 2 => {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/deny-irrefutable-let-patterns/deny-irrefutable-let-patterns.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/deny-irrefutable-let-patterns/deny-irrefutable-let-patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/usefulness/deny-irrefutable-let-patterns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/deny-irrefutable-let-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/deny-irrefutable-let-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/deny-irrefutable-let-patterns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: irrefutable `if let` pattern
   |
   |
LL |     if let _ = 5 {} //~ ERROR irrefutable `if let` pattern
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/pattern/usefulness/deny-irrefutable-let-patterns.rs:3:9
   |
   |
LL | #![deny(irrefutable_let_patterns)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `while let` pattern
   |
   |
LL | /     while let _ = 5 { //~ ERROR irrefutable `while let` pattern
LL | |         break;
LL | |     }
   |
   |
   = note: this pattern will always match, so the loop will never exit
   = help: consider instead using a `loop { ... }` with a `let` inside it

error: irrefutable `if let` guard pattern
   |
   |
LL |         _ if let _ = 2 => {} //~ ERROR irrefutable `if let` guard pattern
   |
   |
   = note: this pattern will always match, so the guard is useless
   = help: consider removing the guard and adding a `let` inside the match arm
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/rfc-2294-if-let-guard/bindings.rs stdout ----
diff of stderr:

1 error[E0425]: cannot find value `y` in this scope
-   --> $DIR/bindings.rs:7:14
3    |
4 LL |         _ => y,
5    |              ^ not found in this scope


6 
7 error[E0425]: cannot find value `y` in this scope
-   --> $DIR/bindings.rs:9:5
9    |
10 LL |     y
11    |     ^ not found in this scope

---
To only update this specific test, also pass `--test-args rfc-2294-if-let-guard/bindings.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2294-if-let-guard/bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/bindings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/bindings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `y` in this scope
   |
   |
LL |         _ => y, //~ ERROR cannot find value `y`
   |              ^ not found in this scope

error[E0425]: cannot find value `y` in this scope
   |
   |
LL |     y //~ ERROR cannot find value `y`
   |     ^ not found in this scope
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.

---
1 error[E0308]: mismatched types
-   --> $DIR/typeck.rs:10:22
+   --> $DIR/typeck.rs:9:22
3    |
4 LL |         Ok(x) if let Err(_) = x => {},
5    |                      ^^^^^^ expected enum `Option`, found enum `Result`
8               found enum `Result<_, _>`
9 
10 error[E0308]: mismatched types
-   --> $DIR/typeck.rs:12:22
-   --> $DIR/typeck.rs:12:22
+   --> $DIR/typeck.rs:11:22
12    |
13 LL |         Ok(x) if let 0 = x => {},
14    |                      ^ expected enum `Option`, found integer

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/typeck/typeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2294-if-let-guard/typeck.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2294-if-let-guard/typeck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/typeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/typeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/typeck.rs:9:22
   |
LL |         Ok(x) if let Err(_) = x => {},
   |                      ^^^^^^ expected enum `Option`, found enum `Result`
   = note: expected enum `Option<bool>`
              found enum `Result<_, _>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/typeck.rs:11:22
   |
LL |         Ok(x) if let 0 = x => {},
   |                      ^ expected enum `Option`, found integer
   = note: expected enum `Option<bool>`
   = note: expected enum `Option<bool>`
              found type `{integer}`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/rfc-2294-if-let-guard/warns.rs stdout ----
diff of stderr:

1 error: irrefutable `if let` guard pattern
-   --> $DIR/warns.rs:7:24
3    |
3    |
4 LL |         Some(x) if let () = x => {}

6    |
7 note: the lint level is defined here
-   --> $DIR/warns.rs:4:8
-   --> $DIR/warns.rs:4:8
+   --> $DIR/warns.rs:3:8
9    |
10 LL | #[deny(irrefutable_let_patterns)]


13    = help: consider removing the guard and adding a `let` inside the match arm
15 error: unreachable pattern
-   --> $DIR/warns.rs:16:25
+   --> $DIR/warns.rs:15:25
17    |
17    |
18 LL |         x if let None | None = x => {}

20    |
21 note: the lint level is defined here
-   --> $DIR/warns.rs:13:8
---
To only update this specific test, also pass `--test-args rfc-2294-if-let-guard/warns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2294-if-let-guard/warns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/warns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/warns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: irrefutable `if let` guard pattern
   |
   |
LL |         Some(x) if let () = x => {}
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/warns.rs:3:8
   |
   |
LL | #[deny(irrefutable_let_patterns)]
   |        ^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this pattern will always match, so the guard is useless
   = help: consider removing the guard and adding a `let` inside the match arm
error: unreachable pattern
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/warns.rs:15:25
   |
   |
LL |         x if let None | None = x => {}
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/warns.rs:12:8
   |
---
test result: FAILED. 12025 passed; 4 failed; 103 ignored; 0 measured; 0 filtered out; finished in 107.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:42
