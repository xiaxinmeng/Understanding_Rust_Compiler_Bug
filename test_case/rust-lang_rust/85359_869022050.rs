plain
............i.i..................................................................................... 12000/12021
.....................
failures:

---- [ui] ui/rust-2021/reserved-prefixes-migration.rs stdout ----

9    |
10 LL | #![warn(reserved_prefix)]
11    |         ^^^^^^^^^^^^^^^
11    |         ^^^^^^^^^^^^^^^
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
13    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
14 help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


22 LL |     m2!(prefix"hey");
23    |         ^^^^^^ unknown prefix
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
26    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
26    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
27 help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021


35 LL |     m3!(hey#123);
36    |         ^^^ unknown prefix
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
39    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
39    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
40 help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021


48 LL |     m3!(hey#hey);
49    |         ^^^ unknown prefix
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
52    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
52    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
53 help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021


61 LL |     #name = #kind#value
62    |              ^^^^ unknown prefix
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
65    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
65    = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
66 help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/reserved-prefixes-migration/reserved-prefixes-migration.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2021/reserved-prefixes-migration.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2021/reserved-prefixes-migration.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/reserved-prefixes-migration" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/reserved-prefixes-migration/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: prefix `z` is unknown
  --> /checkout/src/test/ui/rust-2021/reserved-prefixes-migration.rs:16:9
   |
LL |     m2!(z"hey");
   |         ^ unknown prefix
note: the lint level is defined here
  --> /checkout/src/test/ui/rust-2021/reserved-prefixes-migration.rs:5:9
   |
LL | #![warn(reserved_prefix)]
LL | #![warn(reserved_prefix)]
   |         ^^^^^^^^^^^^^^^
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021
   |
LL |     m2!(z "hey");


warning: prefix `prefix` is unknown
  --> /checkout/src/test/ui/rust-2021/reserved-prefixes-migration.rs:19:9
   |
LL |     m2!(prefix"hey");
   |         ^^^^^^ unknown prefix
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021
   |
LL |     m2!(prefix "hey");


warning: prefix `hey` is unknown
  --> /checkout/src/test/ui/rust-2021/reserved-prefixes-migration.rs:22:9
   |
LL |     m3!(hey#123);
   |         ^^^ unknown prefix
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021
   |
LL |     m3!(hey #123);


warning: prefix `hey` is unknown
  --> /checkout/src/test/ui/rust-2021/reserved-prefixes-migration.rs:25:9
   |
LL |     m3!(hey#hey);
   |         ^^^ unknown prefix
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021
   |
LL |     m3!(hey #hey);

warning: prefix `kind` is unknown
  --> /checkout/src/test/ui/rust-2021/reserved-prefixes-migration.rs:35:14
   |
   |
LL |     #name = #kind#value
   |              ^^^^ unknown prefix
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
   = note: for more information, see issue #84978 <https://github.com/rust-lang/rust/issues/84978>
help: insert whitespace here to avoid this being parsed as a prefix in Rust 2021
   |
LL |     #name = #kind #value

warning: 5 warnings emitted


---
test result: FAILED. 11923 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 127.62s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:59
